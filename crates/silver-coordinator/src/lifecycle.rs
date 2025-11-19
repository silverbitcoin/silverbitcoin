//! Transaction lifecycle management
//!
//! Tracks transaction status from submission through execution to finalization.

use crate::{Error, Result};
use dashmap::DashMap;
use silver_core::{TransactionDigest, TransactionExpiration};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Transaction status in the lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    /// Transaction has been submitted and is pending consensus
    Pending,
    
    /// Transaction is being executed
    Executing,
    
    /// Transaction has been executed successfully
    Executed,
    
    /// Transaction execution failed
    Failed,
    
    /// Transaction has expired
    Expired,
    
    /// Transaction was rejected during validation
    Rejected,
}

impl TransactionStatus {
    /// Check if this is a terminal status
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TransactionStatus::Executed
                | TransactionStatus::Failed
                | TransactionStatus::Expired
                | TransactionStatus::Rejected
        )
    }
    
    /// Check if this status indicates success
    pub fn is_successful(&self) -> bool {
        matches!(self, TransactionStatus::Executed)
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Executing => write!(f, "Executing"),
            TransactionStatus::Executed => write!(f, "Executed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::Expired => write!(f, "Expired"),
            TransactionStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

/// Transaction lifecycle information
#[derive(Debug, Clone)]
pub struct TransactionLifecycle {
    /// Transaction digest
    pub digest: TransactionDigest,
    
    /// Current status
    pub status: TransactionStatus,
    
    /// Submission timestamp (Unix milliseconds)
    pub submitted_at: u64,
    
    /// Execution start timestamp (Unix milliseconds, if started)
    pub execution_started_at: Option<u64>,
    
    /// Finalization timestamp (Unix milliseconds, if finalized)
    pub finalized_at: Option<u64>,
    
    /// Expiration info
    pub expiration: TransactionExpiration,
    
    /// Error message (if failed or rejected)
    pub error: Option<String>,
    
    /// Fuel used (if executed)
    pub fuel_used: Option<u64>,
    
    /// Snapshot number where transaction was finalized (if executed)
    pub snapshot_number: Option<u64>,
}

impl TransactionLifecycle {
    /// Create a new pending transaction lifecycle
    pub fn new_pending(
        digest: TransactionDigest,
        submitted_at: u64,
        expiration: TransactionExpiration,
    ) -> Self {
        Self {
            digest,
            status: TransactionStatus::Pending,
            submitted_at,
            execution_started_at: None,
            finalized_at: None,
            expiration,
            error: None,
            fuel_used: None,
            snapshot_number: None,
        }
    }
    
    /// Mark transaction as executing
    pub fn mark_executing(&mut self, timestamp: u64) {
        self.status = TransactionStatus::Executing;
        self.execution_started_at = Some(timestamp);
    }
    
    /// Mark transaction as executed
    pub fn mark_executed(&mut self, timestamp: u64, fuel_used: u64, snapshot_number: u64) {
        self.status = TransactionStatus::Executed;
        self.finalized_at = Some(timestamp);
        self.fuel_used = Some(fuel_used);
        self.snapshot_number = Some(snapshot_number);
    }
    
    /// Mark transaction as failed
    pub fn mark_failed(&mut self, timestamp: u64, error: String, fuel_used: Option<u64>) {
        self.status = TransactionStatus::Failed;
        self.finalized_at = Some(timestamp);
        self.error = Some(error);
        self.fuel_used = fuel_used;
    }
    
    /// Mark transaction as expired
    pub fn mark_expired(&mut self, timestamp: u64) {
        self.status = TransactionStatus::Expired;
        self.finalized_at = Some(timestamp);
    }
    
    /// Mark transaction as rejected
    pub fn mark_rejected(&mut self, timestamp: u64, error: String) {
        self.status = TransactionStatus::Rejected;
        self.finalized_at = Some(timestamp);
        self.error = Some(error);
    }
    
    /// Check if transaction has expired
    pub fn is_expired(&self, current_time: u64, current_snapshot: u64) -> bool {
        self.expiration.is_expired(current_time, current_snapshot)
    }
    
    /// Get the total time in the system (milliseconds)
    pub fn total_time_ms(&self) -> Option<u64> {
        self.finalized_at.map(|fin| fin.saturating_sub(self.submitted_at))
    }
    
    /// Get the execution time (milliseconds)
    pub fn execution_time_ms(&self) -> Option<u64> {
        match (self.execution_started_at, self.finalized_at) {
            (Some(start), Some(end)) => Some(end.saturating_sub(start)),
            _ => None,
        }
    }
}

/// Transaction lifecycle manager
///
/// Manages the lifecycle of transactions from submission to finalization.
pub struct LifecycleManager {
    /// Active transactions (digest -> lifecycle)
    active_transactions: Arc<DashMap<TransactionDigest, TransactionLifecycle>>,
    
    /// Maximum number of active transactions to track
    max_active_transactions: usize,
    
    /// Current time provider (for testing)
    current_time_fn: Box<dyn Fn() -> u64 + Send + Sync>,
    
    /// Current snapshot provider (for testing)
    current_snapshot_fn: Box<dyn Fn() -> u64 + Send + Sync>,
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new(max_active_transactions: usize) -> Self {
        Self {
            active_transactions: Arc::new(DashMap::new()),
            max_active_transactions,
            current_time_fn: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            current_snapshot_fn: Box::new(|| 0),
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
    
    /// Register a new pending transaction
    pub fn register_pending(
        &self,
        digest: TransactionDigest,
        expiration: TransactionExpiration,
    ) -> Result<()> {
        // Check if we're at capacity
        if self.active_transactions.len() >= self.max_active_transactions {
            // Try to clean up expired transactions first
            self.cleanup_expired();
            
            if self.active_transactions.len() >= self.max_active_transactions {
                return Err(Error::Internal(format!(
                    "Maximum active transactions reached: {}",
                    self.max_active_transactions
                )));
            }
        }
        
        let timestamp = (self.current_time_fn)();
        let lifecycle = TransactionLifecycle::new_pending(digest, timestamp, expiration);
        
        self.active_transactions.insert(digest, lifecycle);
        debug!("Registered pending transaction: {}", digest);
        
        Ok(())
    }
    
    /// Mark a transaction as executing
    pub fn mark_executing(&self, digest: &TransactionDigest) -> Result<()> {
        let timestamp = (self.current_time_fn)();
        
        self.active_transactions
            .get_mut(digest)
            .ok_or_else(|| Error::TransactionNotFound(digest.to_string()))?
            .mark_executing(timestamp);
        
        debug!("Transaction {} marked as executing", digest);
        Ok(())
    }
    
    /// Mark a transaction as executed
    pub fn mark_executed(
        &self,
        digest: &TransactionDigest,
        fuel_used: u64,
        snapshot_number: u64,
    ) -> Result<()> {
        let timestamp = (self.current_time_fn)();
        
        let mut lifecycle = self
            .active_transactions
            .get_mut(digest)
            .ok_or_else(|| Error::TransactionNotFound(digest.to_string()))?;
        
        lifecycle.mark_executed(timestamp, fuel_used, snapshot_number);
        
        info!(
            "Transaction {} executed successfully (fuel: {}, snapshot: {})",
            digest, fuel_used, snapshot_number
        );
        
        Ok(())
    }
    
    /// Mark a transaction as failed
    pub fn mark_failed(
        &self,
        digest: &TransactionDigest,
        error: String,
        fuel_used: Option<u64>,
    ) -> Result<()> {
        let timestamp = (self.current_time_fn)();
        
        let mut lifecycle = self
            .active_transactions
            .get_mut(digest)
            .ok_or_else(|| Error::TransactionNotFound(digest.to_string()))?;
        
        lifecycle.mark_failed(timestamp, error.clone(), fuel_used);
        
        warn!("Transaction {} failed: {}", digest, error);
        
        Ok(())
    }
    
    /// Mark a transaction as expired
    pub fn mark_expired(&self, digest: &TransactionDigest) -> Result<()> {
        let timestamp = (self.current_time_fn)();
        
        let mut lifecycle = self
            .active_transactions
            .get_mut(digest)
            .ok_or_else(|| Error::TransactionNotFound(digest.to_string()))?;
        
        lifecycle.mark_expired(timestamp);
        
        info!("Transaction {} expired", digest);
        
        Ok(())
    }
    
    /// Mark a transaction as rejected
    pub fn mark_rejected(&self, digest: &TransactionDigest, error: String) -> Result<()> {
        let timestamp = (self.current_time_fn)();
        
        let mut lifecycle = self
            .active_transactions
            .get_mut(digest)
            .ok_or_else(|| Error::TransactionNotFound(digest.to_string()))?;
        
        lifecycle.mark_rejected(timestamp, error.clone());
        
        warn!("Transaction {} rejected: {}", digest, error);
        
        Ok(())
    }
    
    /// Get transaction status
    pub fn get_status(&self, digest: &TransactionDigest) -> Option<TransactionStatus> {
        self.active_transactions
            .get(digest)
            .map(|lifecycle| lifecycle.status)
    }
    
    /// Get transaction lifecycle
    pub fn get_lifecycle(&self, digest: &TransactionDigest) -> Option<TransactionLifecycle> {
        self.active_transactions
            .get(digest)
            .map(|lifecycle| lifecycle.clone())
    }
    
    /// Get all pending transactions
    pub fn get_pending_transactions(&self) -> Vec<TransactionDigest> {
        self.active_transactions
            .iter()
            .filter(|entry| entry.value().status == TransactionStatus::Pending)
            .map(|entry| *entry.key())
            .collect()
    }
    
    /// Get all executing transactions
    pub fn get_executing_transactions(&self) -> Vec<TransactionDigest> {
        self.active_transactions
            .iter()
            .filter(|entry| entry.value().status == TransactionStatus::Executing)
            .map(|entry| *entry.key())
            .collect()
    }
    
    /// Clean up expired transactions
    pub fn cleanup_expired(&self) -> usize {
        let current_time = (self.current_time_fn)() / 1000; // Convert to seconds
        let current_snapshot = (self.current_snapshot_fn)();
        
        let expired: Vec<TransactionDigest> = self
            .active_transactions
            .iter()
            .filter(|entry| {
                let lifecycle = entry.value();
                !lifecycle.status.is_terminal()
                    && lifecycle.is_expired(current_time, current_snapshot)
            })
            .map(|entry| *entry.key())
            .collect();
        
        let count = expired.len();
        
        for digest in expired {
            let _ = self.mark_expired(&digest);
        }
        
        if count > 0 {
            info!("Cleaned up {} expired transactions", count);
        }
        
        count
    }
    
    /// Remove finalized transactions older than the given age (milliseconds)
    pub fn prune_old_transactions(&self, max_age_ms: u64) -> usize {
        let current_time = (self.current_time_fn)();
        let cutoff_time = current_time.saturating_sub(max_age_ms);
        
        let to_remove: Vec<TransactionDigest> = self
            .active_transactions
            .iter()
            .filter(|entry| {
                let lifecycle = entry.value();
                lifecycle.status.is_terminal()
                    && lifecycle
                        .finalized_at
                        .map(|t| t < cutoff_time)
                        .unwrap_or(false)
            })
            .map(|entry| *entry.key())
            .collect();
        
        let count = to_remove.len();
        
        for digest in to_remove {
            self.active_transactions.remove(&digest);
        }
        
        if count > 0 {
            debug!("Pruned {} old finalized transactions", count);
        }
        
        count
    }
    
    /// Get statistics about active transactions
    pub fn get_statistics(&self) -> LifecycleStatistics {
        let mut stats = LifecycleStatistics::default();
        
        for entry in self.active_transactions.iter() {
            let lifecycle = entry.value();
            
            match lifecycle.status {
                TransactionStatus::Pending => stats.pending_count += 1,
                TransactionStatus::Executing => stats.executing_count += 1,
                TransactionStatus::Executed => stats.executed_count += 1,
                TransactionStatus::Failed => stats.failed_count += 1,
                TransactionStatus::Expired => stats.expired_count += 1,
                TransactionStatus::Rejected => stats.rejected_count += 1,
            }
            
            if let Some(time) = lifecycle.total_time_ms() {
                stats.total_latency_ms += time;
                stats.latency_sample_count += 1;
            }
        }
        
        stats.total_count = self.active_transactions.len();
        
        stats
    }
}

/// Lifecycle statistics
#[derive(Debug, Clone, Default)]
pub struct LifecycleStatistics {
    /// Total number of tracked transactions
    pub total_count: usize,
    
    /// Number of pending transactions
    pub pending_count: usize,
    
    /// Number of executing transactions
    pub executing_count: usize,
    
    /// Number of executed transactions
    pub executed_count: usize,
    
    /// Number of failed transactions
    pub failed_count: usize,
    
    /// Number of expired transactions
    pub expired_count: usize,
    
    /// Number of rejected transactions
    pub rejected_count: usize,
    
    /// Total latency (milliseconds)
    pub total_latency_ms: u64,
    
    /// Number of samples for latency calculation
    pub latency_sample_count: usize,
}

impl LifecycleStatistics {
    /// Get average latency (milliseconds)
    pub fn average_latency_ms(&self) -> Option<f64> {
        if self.latency_sample_count > 0 {
            Some(self.total_latency_ms as f64 / self.latency_sample_count as f64)
        } else {
            None
        }
    }
    
    /// Get success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        let finalized = self.executed_count + self.failed_count + self.rejected_count;
        if finalized > 0 {
            self.executed_count as f64 / finalized as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silver_core::TransactionDigest;

    #[test]
    fn test_lifecycle_transitions() {
        let manager = LifecycleManager::new(1000)
            .with_time_fn(|| 1000)
            .with_snapshot_fn(|| 10);
        
        let digest = TransactionDigest::new([1u8; 64]);
        
        // Register pending
        manager
            .register_pending(digest, TransactionExpiration::None)
            .unwrap();
        assert_eq!(
            manager.get_status(&digest),
            Some(TransactionStatus::Pending)
        );
        
        // Mark executing
        manager.mark_executing(&digest).unwrap();
        assert_eq!(
            manager.get_status(&digest),
            Some(TransactionStatus::Executing)
        );
        
        // Mark executed
        manager.mark_executed(&digest, 5000, 100).unwrap();
        assert_eq!(
            manager.get_status(&digest),
            Some(TransactionStatus::Executed)
        );
        
        let lifecycle = manager.get_lifecycle(&digest).unwrap();
        assert_eq!(lifecycle.fuel_used, Some(5000));
        assert_eq!(lifecycle.snapshot_number, Some(100));
    }

    #[test]
    fn test_expiration_cleanup() {
        let manager = LifecycleManager::new(1000)
            .with_time_fn(|| 2000000) // 2000 seconds
            .with_snapshot_fn(|| 100);
        
        // Register transaction that expires at timestamp 1000
        let digest = TransactionDigest::new([1u8; 64]);
        manager
            .register_pending(digest, TransactionExpiration::Timestamp(1000))
            .unwrap();
        
        // Clean up expired
        let count = manager.cleanup_expired();
        assert_eq!(count, 1);
        assert_eq!(
            manager.get_status(&digest),
            Some(TransactionStatus::Expired)
        );
    }

    #[test]
    fn test_statistics() {
        let manager = LifecycleManager::new(1000)
            .with_time_fn(|| 1000)
            .with_snapshot_fn(|| 10);
        
        // Add some transactions
        for i in 0..5 {
            let digest = TransactionDigest::new([i; 64]);
            manager
                .register_pending(digest, TransactionExpiration::None)
                .unwrap();
            
            if i < 3 {
                manager.mark_executing(&digest).unwrap();
                manager.mark_executed(&digest, 1000, 10).unwrap();
            }
        }
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_count, 5);
        assert_eq!(stats.pending_count, 2);
        assert_eq!(stats.executed_count, 3);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn test_max_capacity() {
        let manager = LifecycleManager::new(2);
        
        let digest1 = TransactionDigest::new([1u8; 64]);
        let digest2 = TransactionDigest::new([2u8; 64]);
        let digest3 = TransactionDigest::new([3u8; 64]);
        
        manager
            .register_pending(digest1, TransactionExpiration::None)
            .unwrap();
        manager
            .register_pending(digest2, TransactionExpiration::None)
            .unwrap();
        
        // Should fail - at capacity
        let result = manager.register_pending(digest3, TransactionExpiration::None);
        assert!(result.is_err());
    }
}

