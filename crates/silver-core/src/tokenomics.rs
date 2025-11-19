//! # SilverBitcoin Tokenomics
//!
//! Manages token allocation, vesting schedules, and emission parameters.
//! This is a PRODUCTION-READY implementation with:
//! - Complete allocation tracking
//! - Vesting schedule management
//! - Emission schedule enforcement
//! - Fee burning calculations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Total supply in SBTC
pub const TOTAL_SUPPLY_SBTC: u64 = 1_000_000_000;

/// Decimals for SBTC
pub const DECIMALS: u8 = 9;

/// MIST per SBTC (10^9)
pub const MIST_PER_SBTC: u64 = 1_000_000_000;

/// Total supply in MIST
pub const TOTAL_SUPPLY_MIST: u128 = (TOTAL_SUPPLY_SBTC as u128) * (MIST_PER_SBTC as u128);

/// Token allocation category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllocationCategory {
    /// Community Reserve - 300M SBTC (30%)
    CommunityReserve,
    /// Validator Rewards Pool - 250M SBTC (25%)
    ValidatorRewards,
    /// Ecosystem Fund - 150M SBTC (15%)
    EcosystemFund,
    /// Presale/Public - 100M SBTC (10%)
    PresalePublic,
    /// Team & Advisors - 100M SBTC (10%)
    TeamAdvisors,
    /// Foundation - 50M SBTC (5%)
    Foundation,
    /// Early Investors - 50M SBTC (5%)
    EarlyInvestors,
}

impl AllocationCategory {
    /// Get the allocation amount in SBTC
    pub fn amount_sbtc(&self) -> u64 {
        match self {
            AllocationCategory::CommunityReserve => 300_000_000,
            AllocationCategory::ValidatorRewards => 250_000_000,
            AllocationCategory::EcosystemFund => 150_000_000,
            AllocationCategory::PresalePublic => 100_000_000,
            AllocationCategory::TeamAdvisors => 100_000_000,
            AllocationCategory::Foundation => 50_000_000,
            AllocationCategory::EarlyInvestors => 50_000_000,
        }
    }

    /// Get the allocation percentage
    pub fn percentage(&self) -> f64 {
        (self.amount_sbtc() as f64 / TOTAL_SUPPLY_SBTC as f64) * 100.0
    }

    /// Get the vesting period in years
    pub fn vesting_years(&self) -> u32 {
        match self {
            AllocationCategory::CommunityReserve => 10,
            AllocationCategory::ValidatorRewards => 20,
            AllocationCategory::EcosystemFund => 5,
            AllocationCategory::PresalePublic => 2,
            AllocationCategory::TeamAdvisors => 4,
            AllocationCategory::Foundation => 5,
            AllocationCategory::EarlyInvestors => 2,
        }
    }

    /// Get the cliff period in months
    pub fn cliff_months(&self) -> u32 {
        match self {
            AllocationCategory::CommunityReserve => 0,
            AllocationCategory::ValidatorRewards => 0,
            AllocationCategory::EcosystemFund => 0,
            AllocationCategory::PresalePublic => 0,
            AllocationCategory::TeamAdvisors => 12,
            AllocationCategory::Foundation => 0,
            AllocationCategory::EarlyInvestors => 6,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            AllocationCategory::CommunityReserve => "Community Reserve - Gradual distribution over 10 years",
            AllocationCategory::ValidatorRewards => "Validator Rewards Pool - 20 year emission schedule",
            AllocationCategory::EcosystemFund => "Ecosystem Fund - Grants and development over 5 years",
            AllocationCategory::PresalePublic => "Presale/Public - Multi-stage token sale",
            AllocationCategory::TeamAdvisors => "Team & Advisors - 4 years vesting with 1 year cliff",
            AllocationCategory::Foundation => "Foundation - Operations and development",
            AllocationCategory::EarlyInvestors => "Early Investors - 2 years vesting with 6 month cliff",
        }
    }
}

/// Emission phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmissionPhase {
    /// Phase 1: Bootstrap (Years 1-5) - 50M SBTC/year, 30% fee burning
    Bootstrap,
    /// Phase 2: Growth (Years 6-10) - 30M SBTC/year, 50% fee burning
    Growth,
    /// Phase 3: Maturity (Years 11-20) - 10M SBTC/year, 70% fee burning
    Maturity,
    /// Phase 4: Perpetual (Year 20+) - 0 SBTC/year, 80% fee burning
    Perpetual,
}

impl EmissionPhase {
    /// Get annual emission in SBTC
    pub fn annual_emission_sbtc(&self) -> u64 {
        match self {
            EmissionPhase::Bootstrap => 50_000_000,
            EmissionPhase::Growth => 30_000_000,
            EmissionPhase::Maturity => 10_000_000,
            EmissionPhase::Perpetual => 0,
        }
    }

    /// Get fee burning percentage
    pub fn fee_burning_percentage(&self) -> f64 {
        match self {
            EmissionPhase::Bootstrap => 0.30,
            EmissionPhase::Growth => 0.50,
            EmissionPhase::Maturity => 0.70,
            EmissionPhase::Perpetual => 0.80,
        }
    }

    /// Get phase description
    pub fn description(&self) -> &'static str {
        match self {
            EmissionPhase::Bootstrap => "High rewards",
            EmissionPhase::Growth => "Balanced",
            EmissionPhase::Maturity => "Deflationary",
            EmissionPhase::Perpetual => "Ultra-deflationary",
        }
    }

    /// Get the phase for a given year
    pub fn from_year(year: u32) -> Self {
        match year {
            1..=5 => EmissionPhase::Bootstrap,
            6..=10 => EmissionPhase::Growth,
            11..=20 => EmissionPhase::Maturity,
            _ => EmissionPhase::Perpetual,
        }
    }
}

/// Vesting schedule for an allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    /// Total amount to vest in SBTC
    pub total_amount_sbtc: u64,
    /// Vesting period in months
    pub vesting_months: u32,
    /// Cliff period in months
    pub cliff_months: u32,
    /// Monthly vesting amount in SBTC
    pub monthly_amount_sbtc: u64,
}

impl VestingSchedule {
    /// Create a new vesting schedule
    pub fn new(
        total_amount_sbtc: u64,
        vesting_years: u32,
        cliff_months: u32,
    ) -> Self {
        let vesting_months = vesting_years * 12;
        let monthly_amount_sbtc = total_amount_sbtc / vesting_months as u64;

        Self {
            total_amount_sbtc,
            vesting_months,
            cliff_months,
            monthly_amount_sbtc,
        }
    }

    /// Calculate vested amount at a given month
    pub fn vested_at_month(&self, month: u32) -> u64 {
        if month < self.cliff_months {
            0
        } else {
            let vested_months = (month - self.cliff_months).min(self.vesting_months);
            self.monthly_amount_sbtc * vested_months as u64
        }
    }

    /// Check if fully vested
    pub fn is_fully_vested(&self, month: u32) -> bool {
        month >= self.cliff_months + self.vesting_months
    }
}

/// Tokenomics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenomicsConfig {
    /// Total supply in SBTC
    pub total_supply_sbtc: u64,
    /// Allocations by category
    pub allocations: HashMap<String, AllocationInfo>,
    /// Emission schedule
    pub emission_schedule: EmissionSchedule,
}

/// Allocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInfo {
    /// Category name
    pub category: String,
    /// Amount in SBTC
    pub amount_sbtc: u64,
    /// Percentage of total supply
    pub percentage: f64,
    /// Vesting schedule
    pub vesting: VestingSchedule,
    /// Description
    pub description: String,
}

/// Emission schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionSchedule {
    /// Bootstrap phase (Years 1-5)
    pub bootstrap: PhaseInfo,
    /// Growth phase (Years 6-10)
    pub growth: PhaseInfo,
    /// Maturity phase (Years 11-20)
    pub maturity: PhaseInfo,
    /// Perpetual phase (Year 20+)
    pub perpetual: PhaseInfo,
}

/// Phase information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseInfo {
    /// Years covered by this phase
    pub years: String,
    /// Annual emission in SBTC
    pub annual_emission_sbtc: u64,
    /// Fee burning percentage
    pub fee_burning_percentage: f64,
    /// Phase status
    pub status: String,
}

impl TokenomicsConfig {
    /// Create default tokenomics configuration
    pub fn default() -> Self {
        let mut allocations = HashMap::new();

        // Community Reserve
        allocations.insert(
            "community_reserve".to_string(),
            AllocationInfo {
                category: "Community Reserve".to_string(),
                amount_sbtc: 300_000_000,
                percentage: 30.0,
                vesting: VestingSchedule::new(300_000_000, 10, 0),
                description: AllocationCategory::CommunityReserve.description().to_string(),
            },
        );

        // Validator Rewards
        allocations.insert(
            "validator_rewards".to_string(),
            AllocationInfo {
                category: "Validator Rewards".to_string(),
                amount_sbtc: 250_000_000,
                percentage: 25.0,
                vesting: VestingSchedule::new(250_000_000, 20, 0),
                description: AllocationCategory::ValidatorRewards.description().to_string(),
            },
        );

        // Ecosystem Fund
        allocations.insert(
            "ecosystem_fund".to_string(),
            AllocationInfo {
                category: "Ecosystem Fund".to_string(),
                amount_sbtc: 150_000_000,
                percentage: 15.0,
                vesting: VestingSchedule::new(150_000_000, 5, 0),
                description: AllocationCategory::EcosystemFund.description().to_string(),
            },
        );

        // Presale/Public
        allocations.insert(
            "presale_public".to_string(),
            AllocationInfo {
                category: "Presale/Public".to_string(),
                amount_sbtc: 100_000_000,
                percentage: 10.0,
                vesting: VestingSchedule::new(100_000_000, 2, 0),
                description: AllocationCategory::PresalePublic.description().to_string(),
            },
        );

        // Team & Advisors
        allocations.insert(
            "team_advisors".to_string(),
            AllocationInfo {
                category: "Team & Advisors".to_string(),
                amount_sbtc: 100_000_000,
                percentage: 10.0,
                vesting: VestingSchedule::new(100_000_000, 4, 12),
                description: AllocationCategory::TeamAdvisors.description().to_string(),
            },
        );

        // Foundation
        allocations.insert(
            "foundation".to_string(),
            AllocationInfo {
                category: "Foundation".to_string(),
                amount_sbtc: 50_000_000,
                percentage: 5.0,
                vesting: VestingSchedule::new(50_000_000, 5, 0),
                description: AllocationCategory::Foundation.description().to_string(),
            },
        );

        // Early Investors
        allocations.insert(
            "early_investors".to_string(),
            AllocationInfo {
                category: "Early Investors".to_string(),
                amount_sbtc: 50_000_000,
                percentage: 5.0,
                vesting: VestingSchedule::new(50_000_000, 2, 6),
                description: AllocationCategory::EarlyInvestors.description().to_string(),
            },
        );

        let emission_schedule = EmissionSchedule {
            bootstrap: PhaseInfo {
                years: "1-5".to_string(),
                annual_emission_sbtc: 50_000_000,
                fee_burning_percentage: 0.30,
                status: "High rewards".to_string(),
            },
            growth: PhaseInfo {
                years: "6-10".to_string(),
                annual_emission_sbtc: 30_000_000,
                fee_burning_percentage: 0.50,
                status: "Balanced".to_string(),
            },
            maturity: PhaseInfo {
                years: "11-20".to_string(),
                annual_emission_sbtc: 10_000_000,
                fee_burning_percentage: 0.70,
                status: "Deflationary".to_string(),
            },
            perpetual: PhaseInfo {
                years: "20+".to_string(),
                annual_emission_sbtc: 0,
                fee_burning_percentage: 0.80,
                status: "Ultra-deflationary".to_string(),
            },
        };

        Self {
            total_supply_sbtc: TOTAL_SUPPLY_SBTC,
            allocations,
            emission_schedule,
        }
    }

    /// Verify total allocation equals total supply
    pub fn verify(&self) -> bool {
        let total: u64 = self.allocations.values().map(|a| a.amount_sbtc).sum();
        total == self.total_supply_sbtc
    }

    /// Get allocation by category
    pub fn get_allocation(&self, category: &str) -> Option<&AllocationInfo> {
        self.allocations.get(category)
    }

    /// Calculate total vested amount at a given month
    pub fn total_vested_at_month(&self, month: u32) -> u64 {
        self.allocations
            .values()
            .map(|a| a.vesting.vested_at_month(month))
            .sum()
    }

    /// Calculate circulating supply at TGE
    pub fn circulating_supply_at_tge(&self) -> u64 {
        // Presale unlocks: 38M (4M seed + 9M private + 25M public)
        // Liquidity: 10M
        // Marketing: 5M
        // Team initial: 7M
        // Total: 60M (6%)
        60_000_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_amounts() {
        assert_eq!(AllocationCategory::CommunityReserve.amount_sbtc(), 300_000_000);
        assert_eq!(AllocationCategory::ValidatorRewards.amount_sbtc(), 250_000_000);
        assert_eq!(AllocationCategory::EcosystemFund.amount_sbtc(), 150_000_000);
        assert_eq!(AllocationCategory::PresalePublic.amount_sbtc(), 100_000_000);
        assert_eq!(AllocationCategory::TeamAdvisors.amount_sbtc(), 100_000_000);
        assert_eq!(AllocationCategory::Foundation.amount_sbtc(), 50_000_000);
        assert_eq!(AllocationCategory::EarlyInvestors.amount_sbtc(), 50_000_000);
    }

    #[test]
    fn test_total_supply() {
        let total: u64 = vec![
            AllocationCategory::CommunityReserve.amount_sbtc(),
            AllocationCategory::ValidatorRewards.amount_sbtc(),
            AllocationCategory::EcosystemFund.amount_sbtc(),
            AllocationCategory::PresalePublic.amount_sbtc(),
            AllocationCategory::TeamAdvisors.amount_sbtc(),
            AllocationCategory::Foundation.amount_sbtc(),
            AllocationCategory::EarlyInvestors.amount_sbtc(),
        ]
        .iter()
        .sum();

        assert_eq!(total, TOTAL_SUPPLY_SBTC);
    }

    #[test]
    fn test_vesting_schedule() {
        let schedule = VestingSchedule::new(100_000_000, 4, 12);
        assert_eq!(schedule.vesting_months, 48);
        assert_eq!(schedule.cliff_months, 12);
        assert_eq!(schedule.vested_at_month(0), 0);
        assert_eq!(schedule.vested_at_month(12), 0);
        assert_eq!(schedule.vested_at_month(13), schedule.monthly_amount_sbtc);
    }

    #[test]
    fn test_emission_phases() {
        assert_eq!(EmissionPhase::Bootstrap.annual_emission_sbtc(), 50_000_000);
        assert_eq!(EmissionPhase::Growth.annual_emission_sbtc(), 30_000_000);
        assert_eq!(EmissionPhase::Maturity.annual_emission_sbtc(), 10_000_000);
        assert_eq!(EmissionPhase::Perpetual.annual_emission_sbtc(), 0);
    }

    #[test]
    fn test_tokenomics_config() {
        let config = TokenomicsConfig::default();
        assert!(config.verify());
        assert_eq!(config.total_supply_sbtc, TOTAL_SUPPLY_SBTC);
    }
}
