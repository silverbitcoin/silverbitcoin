//! Main transaction coordinator
//!
//! Coordinates transaction flow from submission through consensus to execution.

use crate::{
    Error, LifecycleManager, Result, SponsorshipValidator, SubmissionHandler, TransactionStatus,
};
use silver_core::{Transaction, TransactionDigest};
use silver_storage::ObjectStore;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

/// Coordinator configuration
#[derive(Debug, Clone)]
pub struct CoordinatorConfig {
    /// Maximum number of active transactions to track
    pub max_active_transactions: usize,
    
    /// Maximum age for finalized transactions before pruning (milliseconds)
    pub max_finalized_age_ms: u64,
    
    /// Interval for cleanup tasks (milliseconds)
    pub cleanup_interval_ms: u64,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            max_active_transactions: 100_000,
            max_finalized_age_ms: 3600_000, // 1 hour
            cleanup_interval_ms: 60_000,     // 1 minute
        }
    }
}

/// Transaction coordinator
///
/// Main coordinator that manages the complete transaction lifecycle:
/// 1. Submission and validation
/// 2. Routing to consensus engine
/// 3. Tracking execution status
/// 4. Managing sponsorship and fuel refunds
/// 5. Handling expiration
pub struct TransactionCoordinator {
    /// Configuration
    config: CoordinatorConfig,
    
    /// Submission handler
    submission_handler: SubmissionHandler,
    
    /// Lifecycle manager
    lifecycle_manager: LifecycleManager,
    
    /// Sponsorship validator
    sponsorship_validator: SponsorshipValidator,
    
    /// Object store
    #[allow(dead_code)]
    object_store: Arc<RwLock<ObjectStore>>,
    
    /// Channel for sending transactions to consensus
    consensus_tx: mpsc::UnboundedSender<Transaction>,
    
    /// Channel for receiving execution results
    execution_rx: Arc<RwLock<mpsc::UnboundedReceiver<ExecutionResult>>>,
}

/// Execution result from the execution engine
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Transaction digest
    pub digest: TransactionDigest,
    
    /// Whether execution succeeded
    pub success: bool,
    
    /// Fuel used
    pub fuel_used: u64,
    
    /// Snapshot number where transaction was finalized
    pub snapshot_number: u64,
    
    /// Error message (if failed)
    pub error: Option<String>,
}

impl TransactionCoordinator {
    /// Create a new transaction coordinator
    pub fn new(
        config: CoordinatorConfig,
        object_store: Arc<RwLock<ObjectStore>>,
        consensus_tx: mpsc::UnboundedSender<Transaction>,
        execution_rx: mpsc::UnboundedReceiver<ExecutionResult>,
    ) -> Self {
        let submission_handler = SubmissionHandler::new(object_store.clone());
        let lifecycle_manager = LifecycleManager::new(config.max_active_transactions);
        let sponsorship_validator = SponsorshipValidator::new(object_store.clone());
        
        Self {
            config,
            submission_handler,
            lifecycle_manager,
            sponsorship_validator,
            object_store,
            consensus_tx,
            execution_rx: Arc::new(RwLock::new(execution_rx)),
        }
    }
    
    /// Submit a transaction
    ///
    /// This is the main entry point for transaction submission.
    /// Returns the transaction digest on success.
    pub async fn submit_transaction(&self, transaction: Transaction) -> Result<TransactionDigest> {
        info!("Submitting transaction");
        
        // Step 1: Validate and submit transaction
        let submission_result = self.submission_handler.submit_transaction(transaction.clone()).await?;
        let digest = submission_result.digest;
        
        // Step 2: Register in lifecycle manager
        self.lifecycle_manager
            .register_pending(digest, transaction.data.expiration)?;
        
        // Step 3: For sponsored transactions, validate sponsorship
        if submission_result.is_sponsored {
            match self.sponsorship_validator.validate_sponsorship(&transaction).await {
                Ok(_sponsorship_info) => {
                    debug!("Sponsorship validated for transaction {}", digest);
                    // Store sponsorship info for later refund processing
                    // In a real implementation, we would store this in a separate map
                }
                Err(e) => {
                    error!("Sponsorship validation failed: {}", e);
                    self.lifecycle_manager
                        .mark_rejected(&digest, format!("Sponsorship validation failed: {}", e))?;
                    return Err(e);
                }
            }
        }
        
        // Step 4: Route to consensus engine
        if let Err(e) = self.consensus_tx.send(transaction) {
            error!("Failed to send transaction to consensus: {}", e);
            self.lifecycle_manager
                .mark_rejected(&digest, format!("Failed to route to consensus: {}", e))?;
            return Err(Error::Consensus(format!("Failed to route to consensus: {}", e)));
        }
        
        info!("Transaction {} submitted successfully", digest);
        Ok(digest)
    }
    
    /// Get transaction status
    pub fn get_transaction_status(&self, digest: &TransactionDigest) -> Option<TransactionStatus> {
        self.lifecycle_manager.get_status(digest)
    }
    
    /// Get transaction lifecycle information
    pub fn get_transaction_lifecycle(
        &self,
        digest: &TransactionDigest,
    ) -> Option<crate::lifecycle::TransactionLifecycle> {
        self.lifecycle_manager.get_lifecycle(digest)
    }
    
    /// Process execution results
    ///
    /// This should be called periodically to process execution results from
    /// the execution engine.
    pub async fn process_execution_results(&self) -> Result<usize> {
        let mut count = 0;
        let mut rx = self.execution_rx.write().await;
        
        while let Ok(result) = rx.try_recv() {
            self.handle_execution_result(result).await?;
            count += 1;
        }
        
        if count > 0 {
            debug!("Processed {} execution results", count);
        }
        
        Ok(count)
    }
    
    /// Handle a single execution result
    async fn handle_execution_result(&self, result: ExecutionResult) -> Result<()> {
        debug!("Handling execution result for transaction {}", result.digest);
        
        if result.success {
            // Mark as executed
            self.lifecycle_manager.mark_executed(
                &result.digest,
                result.fuel_used,
                result.snapshot_number,
            )?;
            
            // Process fuel refund for sponsored transactions
            // In a real implementation, we would:
            // 1. Check if this is a sponsored transaction
            // 2. Calculate unused fuel
            // 3. Refund to sponsor
            
            info!(
                "Transaction {} executed successfully (fuel: {}, snapshot: {})",
                result.digest, result.fuel_used, result.snapshot_number
            );
        } else {
            // Mark as failed
            let error_msg = result.error.unwrap_or_else(|| "Unknown error".to_string());
            self.lifecycle_manager.mark_failed(
                &result.digest,
                error_msg.clone(),
                Some(result.fuel_used),
            )?;
            
            warn!(
                "Transaction {} failed: {} (fuel used: {})",
                result.digest, error_msg, result.fuel_used
            );
        }
        
        Ok(())
    }
    
    /// Run cleanup tasks
    ///
    /// This should be called periodically to:
    /// - Clean up expired transactions
    /// - Prune old finalized transactions
    pub async fn run_cleanup(&self) -> Result<()> {
        debug!("Running cleanup tasks");
        
        // Clean up expired transactions
        let expired_count = self.lifecycle_manager.cleanup_expired();
        if expired_count > 0 {
            info!("Cleaned up {} expired transactions", expired_count);
        }
        
        // Prune old finalized transactions
        let pruned_count = self
            .lifecycle_manager
            .prune_old_transactions(self.config.max_finalized_age_ms);
        if pruned_count > 0 {
            info!("Pruned {} old finalized transactions", pruned_count);
        }
        
        Ok(())
    }
    
    /// Start the coordinator background tasks
    ///
    /// This spawns background tasks for:
    /// - Processing execution results
    /// - Running cleanup tasks
    pub fn start_background_tasks(self: Arc<Self>) {
        // Spawn execution result processor
        let coordinator = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = coordinator.process_execution_results().await {
                    error!("Error processing execution results: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
        
        // Spawn cleanup task
        let coordinator = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    coordinator.config.cleanup_interval_ms,
                ))
                .await;
                
                if let Err(e) = coordinator.run_cleanup().await {
                    error!("Error running cleanup: {}", e);
                }
            }
        });
        
        info!("Transaction coordinator background tasks started");
    }
    
    /// Get coordinator statistics
    pub fn get_statistics(&self) -> crate::lifecycle::LifecycleStatistics {
        self.lifecycle_manager.get_statistics()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silver_core::{
        Command, ObjectID, ObjectRef, SequenceNumber, Signature, SignatureScheme, SilverAddress,
        TransactionData, TransactionExpiration, TransactionKind,
    };
    use silver_storage::RocksDatabase;
    use tempfile::TempDir;

    fn create_test_transaction() -> Transaction {
        let sender = SilverAddress::new([1u8; 64]);
        let fuel_payment = ObjectRef::new(
            ObjectID::new([3u8; 64]),
            SequenceNumber::initial(),
            TransactionDigest::new([4u8; 64]),
        );
        
        let data = TransactionData::new(
            sender,
            fuel_payment,
            10000,
            1000,
            TransactionKind::CompositeChain(vec![Command::TransferObjects {
                objects: vec![fuel_payment],
                recipient: sender,
            }]),
            TransactionExpiration::None,
        );
        
        Transaction::new(
            data,
            vec![Signature {
                scheme: SignatureScheme::Dilithium3,
                bytes: vec![0u8; 100],
            }],
        )
    }

    #[tokio::test]
    async fn test_coordinator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let (consensus_tx, _consensus_rx) = mpsc::unbounded_channel();
        let (_execution_tx, execution_rx) = mpsc::unbounded_channel();
        
        let config = CoordinatorConfig::default();
        let _coordinator = TransactionCoordinator::new(
            config,
            object_store,
            consensus_tx,
            execution_rx,
        );
    }

    #[tokio::test]
    async fn test_transaction_submission_flow() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let (consensus_tx, mut consensus_rx) = mpsc::unbounded_channel();
        let (_execution_tx, execution_rx) = mpsc::unbounded_channel();
        
        let config = CoordinatorConfig::default();
        let coordinator = TransactionCoordinator::new(
            config,
            object_store,
            consensus_tx,
            execution_rx,
        );
        
        let tx = create_test_transaction();
        
        // This will fail at fuel validation since we don't have the object in store
        let result = coordinator.submit_transaction(tx).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execution_result_processing() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let (consensus_tx, _consensus_rx) = mpsc::unbounded_channel();
        let (execution_tx, execution_rx) = mpsc::unbounded_channel();
        
        let config = CoordinatorConfig::default();
        let coordinator = TransactionCoordinator::new(
            config,
            object_store,
            consensus_tx,
            execution_rx,
        );
        
        // Register a pending transaction
        let digest = TransactionDigest::new([1u8; 64]);
        coordinator
            .lifecycle_manager
            .register_pending(digest, TransactionExpiration::None)
            .unwrap();
        
        // Send execution result
        execution_tx
            .send(ExecutionResult {
                digest,
                success: true,
                fuel_used: 5000,
                snapshot_number: 100,
                error: None,
            })
            .unwrap();
        
        // Process results
        let count = coordinator.process_execution_results().await.unwrap();
        assert_eq!(count, 1);
        
        // Check status
        let status = coordinator.get_transaction_status(&digest);
        assert_eq!(status, Some(TransactionStatus::Executed));
    }

    #[tokio::test]
    async fn test_cleanup_tasks() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let (consensus_tx, _consensus_rx) = mpsc::unbounded_channel();
        let (_execution_tx, execution_rx) = mpsc::unbounded_channel();
        
        let config = CoordinatorConfig::default();
        let coordinator = TransactionCoordinator::new(
            config,
            object_store,
            consensus_tx,
            execution_rx,
        );
        
        // Register an expired transaction
        let digest = TransactionDigest::new([1u8; 64]);
        coordinator
            .lifecycle_manager
            .register_pending(digest, TransactionExpiration::Timestamp(1000))
            .unwrap();
        
        // Run cleanup (should mark as expired)
        coordinator.run_cleanup().await.unwrap();
        
        // Check status
        let status = coordinator.get_transaction_status(&digest);
        assert_eq!(status, Some(TransactionStatus::Expired));
    }
}

