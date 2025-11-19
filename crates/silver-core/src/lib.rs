//! # SilverBitcoin Core
//!
//! Core types, traits, and primitives for the SilverBitcoin blockchain.
//!
//! This crate provides the fundamental building blocks used throughout the
//! SilverBitcoin ecosystem, including:
//!
//! - Object model (ObjectID, SilverAddress, Object, Owner)
//! - Transaction structures (Transaction, TransactionData, Command)
//! - Consensus types (Batch, Certificate, Snapshot)
//! - Cryptographic primitives (Signature, PublicKey, Hash)
//! - Error types and result wrappers

#![warn(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// Number of MIST per SBTC (1 SBTC = 1,000,000,000 MIST)
///
/// This provides 9 decimal places of precision for SBTC amounts.
/// Similar to Bitcoin's satoshi (1 BTC = 100,000,000 satoshis),
/// but with one extra decimal place for finer granularity.
pub const MIST_PER_SBTC: u64 = 1_000_000_000;

/// Minimum fuel price in MIST per fuel unit
///
/// This is the absolute minimum price that must be paid per fuel unit.
/// At 1000 MIST per fuel unit, this ensures spam prevention while
/// keeping fees affordable.
pub const MIN_FUEL_PRICE_MIST: u64 = 1000;

/// Address types and utilities
pub mod address;

/// Error types and result wrappers
pub mod error;

/// Object model and identifiers
pub mod object;

/// Cryptographic signatures
pub mod signature;

/// Transaction structures and types
pub mod transaction;

/// Consensus types and structures
pub mod consensus;

/// Hash functions and types
pub mod hash;

/// Protocol definitions
pub mod protocol;

/// Tokenomics configuration and allocation management
pub mod tokenomics;

pub use address::SilverAddress;
pub use error::{Error, Result};
pub use object::{Object, ObjectID, ObjectRef, Owner, SequenceNumber};
pub use signature::{PublicKey, Signature, SignatureScheme};
pub use transaction::{Transaction, TransactionData, TransactionKind, Command, TransactionExpiration, Identifier};
pub use consensus::{
    BatchID, Certificate, Snapshot, SnapshotSequenceNumber, TransactionBatch, ValidatorID,
    ValidatorMetadata, ValidatorSignature,
};
pub use hash::{TransactionDigest, SnapshotDigest, StateDigest, Blake3Hash};
pub use tokenomics::{
    TokenomicsConfig, AllocationCategory, EmissionPhase, VestingSchedule,
    TOTAL_SUPPLY_SBTC, TOTAL_SUPPLY_MIST, DECIMALS,
};
pub use protocol::{
    ProtocolVersion, FeatureFlags, UpgradeProposal, ProposalID, UpgradeVote,
    VotingResults, ApprovedUpgrade,
};
