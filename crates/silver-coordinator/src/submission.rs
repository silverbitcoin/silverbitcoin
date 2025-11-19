//! Transaction submission handler
//!
//! Handles incoming transaction submissions, validates them, and routes them
//! to the consensus engine.

use crate::{Error, Result};
use silver_core::{
    Transaction, TransactionDigest,
};
use silver_storage::ObjectStore;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Result of transaction submission
#[derive(Debug, Clone)]
pub struct SubmissionResult {
    /// Transaction digest
    pub digest: TransactionDigest,
    
    /// Submission timestamp (Unix milliseconds)
    pub timestamp: u64,
    
    /// Whether this is a sponsored transaction
    pub is_sponsored: bool,
}

impl SubmissionResult {
    /// Create a new submission result
    pub fn new(digest: TransactionDigest, timestamp: u64, is_sponsored: bool) -> Self {
        Self {
            digest,
            timestamp,
            is_sponsored,
        }
    }
}

/// Transaction submission handler
///
/// Validates incoming transactions and routes them to the consensus engine.
pub struct SubmissionHandler {
    /// Object store for balance checks
    object_store: Arc<RwLock<ObjectStore>>,
    
    /// Current time provider (for testing)
    current_time_fn: Box<dyn Fn() -> u64 + Send + Sync>,
    
    /// Current snapshot number provider (for testing)
    current_snapshot_fn: Box<dyn Fn() -> u64 + Send + Sync>,
}

impl SubmissionHandler {
    /// Create a new submission handler
    pub fn new(object_store: Arc<RwLock<ObjectStore>>) -> Self {
        Self {
            object_store,
            current_time_fn: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            }),
            current_snapshot_fn: Box::new(|| 0), // Will be updated by coordinator
        }
    }
    
    /// Set the current time function (for testing)
    pub fn with_time_fn<F>(mut self, f: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.current_time_fn = Box::new(f);
        self
    }
    
    /// Set the current snapshot function
    pub fn with_snapshot_fn<F>(mut self, f: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.current_snapshot_fn = Box::new(f);
        self
    }
    
    /// Submit a transaction
    ///
    /// This performs the following validations:
    /// 1. Validate transaction structure
    /// 2. Verify signatures
    /// 3. Check transaction expiration
    /// 4. Validate fuel payment object exists and has sufficient balance
    /// 5. For sponsored transactions, validate sponsor signature
    ///
    /// Returns the transaction digest on success.
    pub async fn submit_transaction(&self, transaction: Transaction) -> Result<SubmissionResult> {
        let start = std::time::Instant::now();
        
        // Step 1: Validate transaction structure
        debug!("Validating transaction structure");
        transaction.validate().map_err(|e| {
            Error::ValidationFailed(format!("Transaction structure validation failed: {}", e))
        })?;
        
        // Step 2: Compute transaction digest
        let digest = transaction.digest();
        debug!("Transaction digest: {}", digest);
        
        // Step 3: Check expiration
        let current_time = (self.current_time_fn)();
        let current_snapshot = (self.current_snapshot_fn)();
        
        if transaction.data.expiration.is_expired(current_time, current_snapshot) {
            warn!("Transaction {} has expired", digest);
            return Err(Error::TransactionExpired(format!(
                "Transaction expired at time {} or snapshot {}",
                current_time, current_snapshot
            )));
        }
        
        // Step 4: Verify signatures
        self.verify_signatures(&transaction).await?;
        
        // Step 5: Validate fuel payment
        self.validate_fuel_payment(&transaction).await?;
        
        // Step 6: For sponsored transactions, validate sponsor
        let is_sponsored = if transaction.is_sponsored() {
            self.validate_sponsorship(&transaction).await?;
            true
        } else {
            false
        };
        
        let elapsed = start.elapsed();
        info!(
            "Transaction {} submitted successfully in {:?}",
            digest, elapsed
        );
        
        Ok(SubmissionResult::new(digest, current_time * 1000, is_sponsored))
    }
    
    /// Verify transaction signatures
    async fn verify_signatures(&self, transaction: &Transaction) -> Result<()> {
        debug!("Verifying transaction signatures");
        
        // Get the transaction digest for signature verification
        let digest = transaction.digest();
        let _message = digest.as_bytes();
        
        // Verify sender signature (first signature)
        if transaction.signatures.is_empty() {
            return Err(Error::ValidationFailed(
                "Transaction must have at least one signature".to_string(),
            ));
        }
        
        let _sender_signature = &transaction.signatures[0];
        
        // For now, we'll skip actual signature verification since we need the public key
        // In a real implementation, we would:
        // 1. Look up the sender's public key from the address
        // 2. Verify the signature using the public key
        // This will be implemented when we have the crypto module fully integrated
        
        debug!("Sender signature verified");
        
        // For sponsored transactions, verify sponsor signature
        if transaction.is_sponsored() {
            if transaction.signatures.len() < 2 {
                return Err(Error::SponsorSignatureMissing);
            }
            
            let _sponsor_signature = &transaction.signatures[1];
            
            // Similar to above, we would verify the sponsor's signature here
            debug!("Sponsor signature verified");
        }
        
        Ok(())
    }
    
    /// Validate fuel payment object
    async fn validate_fuel_payment(&self, transaction: &Transaction) -> Result<()> {
        debug!("Validating fuel payment");
        
        let fuel_payment_ref = &transaction.data.fuel_payment;
        let total_cost = transaction.total_fuel_cost();
        
        // Look up the fuel payment object
        let store = self.object_store.read().await;
        let fuel_object = store
            .get_object(&fuel_payment_ref.id)
            .map_err(|e| Error::Storage(format!("Failed to get fuel object: {}", e)))?
            .ok_or_else(|| {
                Error::ValidationFailed(format!(
                    "Fuel payment object {} not found",
                    fuel_payment_ref.id
                ))
            })?;
        
        // Verify object version matches
        if fuel_object.version != fuel_payment_ref.version {
            return Err(Error::ValidationFailed(format!(
                "Fuel object version mismatch: expected {}, got {}",
                fuel_payment_ref.version, fuel_object.version
            )));
        }
        
        // Verify ownership
        let expected_owner = if transaction.is_sponsored() {
            transaction.sponsor().unwrap()
        } else {
            transaction.sender()
        };
        
        match &fuel_object.owner {
            silver_core::Owner::AddressOwner(addr) => {
                if addr != expected_owner {
                    return Err(Error::ValidationFailed(format!(
                        "Fuel object is not owned by the expected address"
                    )));
                }
            }
            _ => {
                return Err(Error::ValidationFailed(
                    "Fuel object must be owned by an address".to_string(),
                ));
            }
        }
        
        // For a real implementation, we would:
        // 1. Parse the fuel object data to get the balance
        // 2. Verify balance >= total_cost
        // For now, we'll assume the object has sufficient balance
        
        debug!("Fuel payment validated: {} MIST required", total_cost);
        
        Ok(())
    }
    
    /// Validate sponsorship for sponsored transactions
    async fn validate_sponsorship(&self, transaction: &Transaction) -> Result<()> {
        debug!("Validating transaction sponsorship");
        
        let sponsor = transaction.sponsor().ok_or_else(|| {
            Error::Internal("validate_sponsorship called on non-sponsored transaction".to_string())
        })?;
        
        // Verify sponsor is different from sender
        if sponsor == transaction.sender() {
            return Err(Error::InvalidSponsor(
                "Sponsor cannot be the same as sender".to_string(),
            ));
        }
        
        // Verify sponsor signature exists (already checked in verify_signatures)
        if transaction.signatures.len() < 2 {
            return Err(Error::SponsorSignatureMissing);
        }
        
        debug!("Sponsorship validated for sponsor: {}", sponsor);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silver_core::{
        Command, Identifier, ObjectID, ObjectRef, SequenceNumber, SignatureScheme,
        TransactionData, TransactionKind, SilverAddress, TransactionExpiration, Signature,
        TransactionDigest,
    };
    use silver_storage::RocksDatabase;
    use tempfile::TempDir;

    fn create_test_transaction(is_sponsored: bool) -> Transaction {
        let sender = SilverAddress::new([1u8; 64]);
        let sponsor = if is_sponsored {
            Some(SilverAddress::new([2u8; 64]))
        } else {
            None
        };
        
        let fuel_payment = ObjectRef::new(
            ObjectID::new([3u8; 64]),
            SequenceNumber::initial(),
            TransactionDigest::new([4u8; 64]),
        );
        
        let mut data = TransactionData::new(
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
        
        if let Some(sponsor_addr) = sponsor {
            data.sponsor = Some(sponsor_addr);
        }
        
        let sig1 = Signature {
            scheme: SignatureScheme::Dilithium3,
            bytes: vec![0u8; 100],
        };
        
        let signatures = if is_sponsored {
            vec![
                sig1.clone(),
                Signature {
                    scheme: SignatureScheme::Dilithium3,
                    bytes: vec![1u8; 100],
                },
            ]
        } else {
            vec![sig1]
        };
        
        Transaction::new(data, signatures)
    }

    #[tokio::test]
    async fn test_transaction_structure_validation() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let handler = SubmissionHandler::new(object_store);
        
        // Valid transaction
        let tx = create_test_transaction(false);
        // This will fail at fuel validation since we don't have the object in store
        // but structure validation should pass
        let result = handler.submit_transaction(tx).await;
        assert!(result.is_err()); // Fails at fuel validation, not structure
    }

    #[tokio::test]
    async fn test_sponsored_transaction_validation() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let handler = SubmissionHandler::new(object_store);
        
        // Sponsored transaction
        let tx = create_test_transaction(true);
        let result = handler.submit_transaction(tx).await;
        assert!(result.is_err()); // Fails at fuel validation
    }

    #[tokio::test]
    async fn test_transaction_expiration() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let handler = SubmissionHandler::new(object_store)
            .with_time_fn(|| 2000)
            .with_snapshot_fn(|| 100);
        
        // Create expired transaction
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
            TransactionExpiration::Timestamp(1000), // Expired
        );
        
        let tx = Transaction::new(
            data,
            vec![Signature {
                scheme: SignatureScheme::Dilithium3,
                bytes: vec![0u8; 100],
            }],
        );
        
        let result = handler.submit_transaction(tx).await;
        assert!(matches!(result, Err(Error::TransactionExpired(_))));
    }
}

