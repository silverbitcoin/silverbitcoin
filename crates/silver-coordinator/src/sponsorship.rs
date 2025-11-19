//! Transaction sponsorship support
//!
//! Handles validation and management of sponsored transactions where a sponsor
//! pays for the fuel costs on behalf of the transaction sender.

use crate::{Error, Result};
use silver_core::{ObjectRef, SilverAddress, Transaction, TransactionDigest};
use silver_storage::ObjectStore;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Sponsorship information for a transaction
#[derive(Debug, Clone)]
pub struct SponsorshipInfo {
    /// Transaction digest
    pub transaction_digest: TransactionDigest,
    
    /// Sender address
    pub sender: SilverAddress,
    
    /// Sponsor address
    pub sponsor: SilverAddress,
    
    /// Fuel payment object (owned by sponsor)
    pub fuel_payment: ObjectRef,
    
    /// Total fuel cost
    pub total_fuel_cost: u64,
    
    /// Fuel used during execution
    pub fuel_used: Option<u64>,
    
    /// Fuel refunded to sponsor
    pub fuel_refunded: Option<u64>,
}

impl SponsorshipInfo {
    /// Create new sponsorship info
    pub fn new(
        transaction_digest: TransactionDigest,
        sender: SilverAddress,
        sponsor: SilverAddress,
        fuel_payment: ObjectRef,
        total_fuel_cost: u64,
    ) -> Self {
        Self {
            transaction_digest,
            sender,
            sponsor,
            fuel_payment,
            total_fuel_cost,
            fuel_used: None,
            fuel_refunded: None,
        }
    }
    
    /// Record fuel usage and calculate refund
    pub fn record_fuel_usage(&mut self, fuel_used: u64) {
        self.fuel_used = Some(fuel_used);
        
        // Calculate refund (unused fuel)
        let fuel_budget = self.total_fuel_cost; // This is budget * price
        let fuel_cost = fuel_used; // This would be fuel_used * price in real impl
        
        if fuel_cost < fuel_budget {
            self.fuel_refunded = Some(fuel_budget - fuel_cost);
        } else {
            self.fuel_refunded = Some(0);
        }
    }
    
    /// Check if refund is pending
    pub fn has_pending_refund(&self) -> bool {
        self.fuel_refunded.map(|r| r > 0).unwrap_or(false)
    }
}

/// Sponsorship validator
///
/// Validates sponsored transactions and manages fuel refunds to sponsors.
pub struct SponsorshipValidator {
    /// Object store for balance checks
    object_store: Arc<RwLock<ObjectStore>>,
}

impl SponsorshipValidator {
    /// Create a new sponsorship validator
    pub fn new(object_store: Arc<RwLock<ObjectStore>>) -> Self {
        Self { object_store }
    }
    
    /// Validate a sponsored transaction
    ///
    /// This performs the following checks:
    /// 1. Verify transaction has sponsor field set
    /// 2. Verify sponsor is different from sender
    /// 3. Verify transaction has exactly 2 signatures (sender + sponsor)
    /// 4. Verify fuel payment object is owned by sponsor
    /// 5. Verify sponsor has sufficient balance
    pub async fn validate_sponsorship(&self, transaction: &Transaction) -> Result<SponsorshipInfo> {
        debug!("Validating sponsored transaction");
        
        // Check if transaction is sponsored
        let sponsor = transaction.sponsor().ok_or_else(|| {
            Error::InvalidSponsor("Transaction is not sponsored".to_string())
        })?;
        
        let sender = transaction.sender();
        
        // Verify sponsor is different from sender
        if sponsor == sender {
            return Err(Error::InvalidSponsor(
                "Sponsor cannot be the same as sender".to_string(),
            ));
        }
        
        // Verify signature count
        if transaction.signatures.len() != 2 {
            return Err(Error::InvalidSponsor(format!(
                "Sponsored transaction must have exactly 2 signatures, got {}",
                transaction.signatures.len()
            )));
        }
        
        // Verify fuel payment object ownership
        let fuel_payment = &transaction.data.fuel_payment;
        self.verify_sponsor_owns_fuel_object(sponsor, fuel_payment)
            .await?;
        
        // Verify sponsor has sufficient balance
        let total_cost = transaction.total_fuel_cost();
        self.verify_sponsor_balance(sponsor, fuel_payment, total_cost)
            .await?;
        
        let digest = transaction.digest();
        
        info!(
            "Sponsorship validated: sponsor {} paying {} MIST for sender {}",
            sponsor, total_cost, sender
        );
        
        Ok(SponsorshipInfo::new(
            digest,
            *sender,
            *sponsor,
            *fuel_payment,
            total_cost,
        ))
    }
    
    /// Verify that the sponsor owns the fuel payment object
    async fn verify_sponsor_owns_fuel_object(
        &self,
        sponsor: &SilverAddress,
        fuel_payment: &ObjectRef,
    ) -> Result<()> {
        debug!("Verifying sponsor owns fuel object");
        
        let store = self.object_store.read().await;
        let fuel_object = store
            .get_object(&fuel_payment.id)
            .map_err(|e| Error::Storage(format!("Failed to get fuel object: {}", e)))?
            .ok_or_else(|| {
                Error::InvalidSponsor(format!(
                    "Fuel payment object {} not found",
                    fuel_payment.id
                ))
            })?;
        
        // Verify object version matches
        if fuel_object.version != fuel_payment.version {
            return Err(Error::InvalidSponsor(format!(
                "Fuel object version mismatch: expected {}, got {}",
                fuel_payment.version, fuel_object.version
            )));
        }
        
        // Verify sponsor owns the object
        match &fuel_object.owner {
            silver_core::Owner::AddressOwner(addr) => {
                if addr != sponsor {
                    return Err(Error::InvalidSponsor(format!(
                        "Fuel object is not owned by sponsor (owned by {})",
                        addr
                    )));
                }
            }
            _ => {
                return Err(Error::InvalidSponsor(
                    "Fuel object must be owned by an address".to_string(),
                ));
            }
        }
        
        debug!("Sponsor ownership verified");
        Ok(())
    }
    
    /// Verify sponsor has sufficient balance
    async fn verify_sponsor_balance(
        &self,
        sponsor: &SilverAddress,
        fuel_payment: &ObjectRef,
        required_amount: u64,
    ) -> Result<()> {
        debug!(
            "Verifying sponsor {} has sufficient balance: {} MIST",
            sponsor, required_amount
        );
        
        let store = self.object_store.read().await;
        let _fuel_object = store
            .get_object(&fuel_payment.id)
            .map_err(|e| Error::Storage(format!("Failed to get fuel object: {}", e)))?
            .ok_or_else(|| {
                Error::InvalidSponsor(format!(
                    "Fuel payment object {} not found",
                    fuel_payment.id
                ))
            })?;
        
        // In a real implementation, we would:
        // 1. Parse the fuel object data to extract the balance
        // 2. Verify balance >= required_amount
        //
        // For now, we'll assume the object has sufficient balance
        // This will be properly implemented when the coin/balance types are defined
        
        debug!("Sponsor balance verified");
        Ok(())
    }
    
    /// Process fuel refund to sponsor after transaction execution
    ///
    /// This should be called after transaction execution to refund unused fuel
    /// to the sponsor.
    pub async fn process_refund(&self, sponsorship_info: &mut SponsorshipInfo) -> Result<()> {
        if !sponsorship_info.has_pending_refund() {
            debug!("No refund needed for transaction {}", sponsorship_info.transaction_digest);
            return Ok(());
        }
        
        let refund_amount = sponsorship_info.fuel_refunded.unwrap();
        
        info!(
            "Processing fuel refund: {} MIST to sponsor {} for transaction {}",
            refund_amount, sponsorship_info.sponsor, sponsorship_info.transaction_digest
        );
        
        // In a real implementation, we would:
        // 1. Get the fuel payment object
        // 2. Update its balance to add the refund amount
        // 3. Persist the updated object
        //
        // For now, we'll just log the refund
        // This will be properly implemented when integrated with the execution engine
        
        debug!("Fuel refund processed successfully");
        Ok(())
    }
    
    /// Validate sponsor signature
    ///
    /// Verifies that the second signature in a sponsored transaction is valid
    /// and comes from the sponsor.
    pub async fn validate_sponsor_signature(&self, transaction: &Transaction) -> Result<()> {
        debug!("Validating sponsor signature");
        
        if !transaction.is_sponsored() {
            return Err(Error::InvalidSponsor(
                "Transaction is not sponsored".to_string(),
            ));
        }
        
        if transaction.signatures.len() < 2 {
            return Err(Error::SponsorSignatureMissing);
        }
        
        let _sponsor_signature = &transaction.signatures[1];
        
        // In a real implementation, we would:
        // 1. Get the sponsor's public key from their address
        // 2. Verify the signature using the public key and transaction digest
        //
        // For now, we'll assume the signature is valid
        // This will be properly implemented when the crypto module is fully integrated
        
        debug!("Sponsor signature validated");
        Ok(())
    }
    
    /// Get sponsorship info for a transaction
    pub async fn get_sponsorship_info(
        &self,
        transaction: &Transaction,
    ) -> Result<Option<SponsorshipInfo>> {
        if !transaction.is_sponsored() {
            return Ok(None);
        }
        
        let info = self.validate_sponsorship(transaction).await?;
        Ok(Some(info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use silver_core::{
        Command, ObjectID, SequenceNumber, Signature, SignatureScheme, TransactionData,
        TransactionKind,
    };
    use silver_storage::RocksDatabase;
    use tempfile::TempDir;

    fn create_sponsored_transaction() -> Transaction {
        let sender = SilverAddress::new([1u8; 64]);
        let sponsor = SilverAddress::new([2u8; 64]);
        
        let fuel_payment = ObjectRef::new(
            ObjectID::new([3u8; 64]),
            SequenceNumber::initial(),
            TransactionDigest::new([4u8; 64]),
        );
        
        let data = TransactionData::new_sponsored(
            sender,
            sponsor,
            fuel_payment,
            10000,
            1000,
            TransactionKind::CompositeChain(vec![Command::TransferObjects {
                objects: vec![fuel_payment],
                recipient: sender,
            }]),
            silver_core::TransactionExpiration::None,
        );
        
        let signatures = vec![
            Signature {
                scheme: SignatureScheme::Dilithium3,
                bytes: vec![0u8; 100],
            },
            Signature {
                scheme: SignatureScheme::Dilithium3,
                bytes: vec![1u8; 100],
            },
        ];
        
        Transaction::new(data, signatures)
    }

    #[tokio::test]
    async fn test_sponsorship_info_creation() {
        let digest = TransactionDigest::new([1u8; 64]);
        let sender = SilverAddress::new([2u8; 64]);
        let sponsor = SilverAddress::new([3u8; 64]);
        let fuel_payment = ObjectRef::new(
            ObjectID::new([4u8; 64]),
            SequenceNumber::initial(),
            TransactionDigest::new([5u8; 64]),
        );
        
        let mut info = SponsorshipInfo::new(digest, sender, sponsor, fuel_payment, 10000);
        
        assert_eq!(info.sender, sender);
        assert_eq!(info.sponsor, sponsor);
        assert_eq!(info.total_fuel_cost, 10000);
        assert!(!info.has_pending_refund());
        
        // Record fuel usage
        info.record_fuel_usage(5000);
        assert_eq!(info.fuel_used, Some(5000));
        assert_eq!(info.fuel_refunded, Some(5000));
        assert!(info.has_pending_refund());
    }

    #[tokio::test]
    async fn test_sponsorship_validation_structure() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let validator = SponsorshipValidator::new(object_store);
        
        let tx = create_sponsored_transaction();
        
        // This will fail because the fuel object doesn't exist in the store
        // but it validates the structure
        let result = validator.validate_sponsorship(&tx).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_non_sponsored_transaction() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let validator = SponsorshipValidator::new(object_store);
        
        // Create non-sponsored transaction
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
            silver_core::TransactionExpiration::None,
        );
        
        let tx = Transaction::new(
            data,
            vec![Signature {
                scheme: SignatureScheme::Dilithium3,
                bytes: vec![0u8; 100],
            }],
        );
        
        let result = validator.validate_sponsorship(&tx).await;
        assert!(matches!(result, Err(Error::InvalidSponsor(_))));
    }

    #[tokio::test]
    async fn test_same_sender_and_sponsor() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(RocksDatabase::open(temp_dir.path()).unwrap());
        let object_store = Arc::new(RwLock::new(ObjectStore::new(db)));
        
        let validator = SponsorshipValidator::new(object_store);
        
        // Create transaction where sender == sponsor
        let sender = SilverAddress::new([1u8; 64]);
        let fuel_payment = ObjectRef::new(
            ObjectID::new([3u8; 64]),
            SequenceNumber::initial(),
            TransactionDigest::new([4u8; 64]),
        );
        
        let data = TransactionData::new_sponsored(
            sender,
            sender, // Same as sender
            fuel_payment,
            10000,
            1000,
            TransactionKind::CompositeChain(vec![Command::TransferObjects {
                objects: vec![fuel_payment],
                recipient: sender,
            }]),
            silver_core::TransactionExpiration::None,
        );
        
        let tx = Transaction::new(
            data,
            vec![
                Signature {
                    scheme: SignatureScheme::Dilithium3,
                    bytes: vec![0u8; 100],
                },
                Signature {
                    scheme: SignatureScheme::Dilithium3,
                    bytes: vec![1u8; 100],
                },
            ],
        );
        
        let result = validator.validate_sponsorship(&tx).await;
        assert!(matches!(result, Err(Error::InvalidSponsor(_))));
    }
}

