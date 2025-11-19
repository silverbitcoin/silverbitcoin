//! # SilverBitcoin Transaction Coordinator
//!
//! Transaction coordinator that manages the lifecycle of transactions from
//! submission to finalization.
//!
//! This crate provides:
//! - Transaction submission and validation
//! - Transaction lifecycle management (pending, executed, failed)
//! - Transaction sponsorship support
//! - Coordination between consensus and execution engines
//! - Transaction expiration handling
//! - Fuel refund management

#![warn(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(missing_docs)] // Internal implementation details

mod error;
mod submission;
mod lifecycle;
mod sponsorship;
mod coordinator;

pub use error::{Error, Result};
pub use submission::{SubmissionHandler, SubmissionResult};
pub use lifecycle::{LifecycleManager, TransactionStatus};
pub use sponsorship::{SponsorshipValidator, SponsorshipInfo};
pub use coordinator::{TransactionCoordinator, CoordinatorConfig};

