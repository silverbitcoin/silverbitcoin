//! Error types for transaction coordinator

use thiserror::Error;

/// Coordinator error type
#[derive(Error, Debug)]
pub enum Error {
    /// Transaction validation failed
    #[error("Transaction validation failed: {0}")]
    ValidationFailed(String),
    
    /// Transaction already exists
    #[error("Transaction already exists: {0}")]
    DuplicateTransaction(String),
    
    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    /// Transaction expired
    #[error("Transaction expired: {0}")]
    TransactionExpired(String),
    
    /// Insufficient fuel balance
    #[error("Insufficient fuel balance: required {required}, available {available}")]
    InsufficientFuel { required: u64, available: u64 },
    
    /// Invalid sponsor
    #[error("Invalid sponsor: {0}")]
    InvalidSponsor(String),
    
    /// Sponsor signature missing
    #[error("Sponsor signature missing")]
    SponsorSignatureMissing,
    
    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    /// Consensus error
    #[error("Consensus error: {0}")]
    Consensus(String),
    
    /// Execution error
    #[error("Execution error: {0}")]
    Execution(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// Core error
    #[error("Core error: {0}")]
    Core(#[from] silver_core::Error),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

