//! Error types for SilverBitcoin core

use thiserror::Error;

/// Core error type
#[derive(Error, Debug)]
pub enum Error {
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Invalid data error
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
    
    /// Resource exhausted error
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    
    /// Cryptographic error
    #[error("Cryptographic error: {0}")]
    Cryptographic(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
