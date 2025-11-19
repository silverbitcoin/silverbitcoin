#!/usr/bin/env python3
"""
SilverBitcoin Tokenomics Generator

Generates initial coin allocations based on tokenomics specification:
- Community Reserve: 300M SBTC (30%) - 10 years gradual
- Validator Rewards Pool: 250M SBTC (25%) - 20 years emission
- Ecosystem Fund: 150M SBTC (15%) - 5 years grants
- Presale/Public: 100M SBTC (10%) - Multi-stage
- Team & Advisors: 100M SBTC (10%) - 4 years (1yr cliff)
- Foundation: 50M SBTC (5%) - Operations
- Early Investors: 50M SBTC (5%) - 2 years (6mo cliff)

Total: 1,000,000,000 SBTC (1 Billion - Hard Cap)
"""

import json
import sys
from datetime import datetime, timedelta
from typing import Dict, List, Any

# Tokenomics configuration
TOTAL_SUPPLY = 1_000_000_000  # 1 Billion SBTC
DECIMALS = 9
MIST_PER_SBTC = 10 ** DECIMALS

# Allocation percentages and amounts
ALLOCATIONS = {
    "community_reserve": {
        "amount_sbtc": 300_000_000,
        "percentage": 30,
        "vesting_years": 10,
        "cliff_months": 0,
        "description": "Community Reserve - Gradual distribution over 10 years"
    },
    "validator_rewards": {
        "amount_sbtc": 250_000_000,
        "percentage": 25,
        "vesting_years": 20,
        "cliff_months": 0,
        "description": "Validator Rewards Pool - 20 year emission schedule"
    },
    "ecosystem_fund": {
        "amount_sbtc": 150_000_000,
        "percentage": 15,
        "vesting_years": 5,
        "cliff_months": 0,
        "description": "Ecosystem Fund - Grants and development over 5 years"
    },
    "presale_public": {
        "amount_sbtc": 100_000_000,
        "percentage": 10,
        "vesting_years": 2,
        "cliff_months": 0,
        "description": "Presale/Public - Multi-stage token sale"
    },
    "team_advisors": {
        "amount_sbtc": 100_000_000,
        "percentage": 10,
        "vesting_years": 4,
        "cliff_months": 12,
        "description": "Team & Advisors - 4 years vesting with 1 year cliff"
    },
    "foundation": {
        "amount_sbtc": 50_000_000,
        "percentage": 5,
        "vesting_years": 5,
        "cliff_months": 0,
        "description": "Foundation - Operations and development"
    },
    "early_investors": {
        "amount_sbtc": 50_000_000,
        "percentage": 5,
        "vesting_years": 2,
        "cliff_months": 6,
        "description": "Early Investors - 2 years vesting with 6 month cliff"
    }
}

# Presale breakdown
PRESALE_BREAKDOWN = {
    "seed_round": {
        "amount_sbtc": 20_000_000,
        "price_per_sbtc": 0.33,
        "bonus_percentage": 30,
        "tge_unlock": 0.20,
        "vesting_months": 12,
        "description": "Seed Round"
    },
    "private_sale": {
        "amount_sbtc": 30_000_000,
        "price_per_sbtc": 0.50,
        "bonus_percentage": 20,
        "tge_unlock": 0.30,
        "vesting_months": 8,
        "description": "Private Sale"
    },
    "public_presale": {
        "amount_sbtc": 50_000_000,
        "price_per_sbtc": 0.50,
        "bonus_percentage": 10,
        "tge_unlock": 0.50,
        "vesting_months": 4,
        "description": "Public Presale"
    }
}

# TGE configuration
TGE_CONFIG = {
    "listing_price": 3.00,
    "seed_unlock": 4_000_000,  # 20% of 20M
    "private_unlock": 9_000_000,  # 30% of 30M
    "public_unlock": 25_000_000,  # 50% of 50M
    "liquidity_pool": 10_000_000,
    "marketing_airdrops": 5_000_000,
    "team_initial": 7_000_000,
}

def calculate_vesting_schedule(
    total_amount: int,
    vesting_years: int,
    cliff_months: int,
    start_date: datetime
) -> List[Dict[str, Any]]:
    """Calculate monthly vesting schedule."""
    schedule = []
    cliff_date = start_date + timedelta(days=cliff_months * 30)
    monthly_amount = total_amount // (vesting_years * 12)
    
    for month in range(vesting_years * 12):
        release_date = start_date + timedelta(days=month * 30)
        
        if release_date < cliff_date:
            amount = 0
        else:
            amount = monthly_amount
        
        schedule.append({
            "month": month + 1,
            "date": release_date.isoformat(),
            "amount_sbtc": amount,
            "amount_mist": amount * MIST_PER_SBTC,
            "cumulative_sbtc": sum(s["amount_sbtc"] for s in schedule) + amount
        })
    
    return schedule

def generate_allocation_accounts() -> List[Dict[str, Any]]:
    """Generate allocation accounts for each category."""
    accounts = []
    start_date = datetime(2024, 12, 1)  # Testnet launch
    
    for category, config in ALLOCATIONS.items():
        amount_mist = config["amount_sbtc"] * MIST_PER_SBTC
        
        # Generate deterministic address based on category
        address_suffix = hex(hash(category) & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)[2:].zfill(64)
        address = f"0x{address_suffix}"
        
        vesting_schedule = calculate_vesting_schedule(
            config["amount_sbtc"],
            config["vesting_years"],
            config["cliff_months"],
            start_date
        )
        
        account = {
            "category": category,
            "address": address,
            "balance_sbtc": config["amount_sbtc"],
            "balance_mist": amount_mist,
            "percentage": config["percentage"],
            "description": config["description"],
            "vesting": {
                "years": config["vesting_years"],
                "cliff_months": config["cliff_months"],
                "start_date": start_date.isoformat(),
                "schedule": vesting_schedule
            }
        }
        accounts.append(account)
    
    return accounts

def generate_presale_accounts() -> List[Dict[str, Any]]:
    """Generate presale participant accounts."""
    accounts = []
    
    for round_name, config in PRESALE_BREAKDOWN.items():
        amount_mist = config["amount_sbtc"] * MIST_PER_SBTC
        tge_unlock_mist = int(config["amount_sbtc"] * config["tge_unlock"] * MIST_PER_SBTC)
        
        # Generate deterministic address
        address_suffix = hex(hash(f"presale_{round_name}") & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)[2:].zfill(64)
        address = f"0x{address_suffix}"
        
        total_raise = config["amount_sbtc"] * config["price_per_sbtc"]
        
        account = {
            "round": round_name,
            "address": address,
            "allocation_sbtc": config["amount_sbtc"],
            "allocation_mist": amount_mist,
            "price_per_sbtc": config["price_per_sbtc"],
            "bonus_percentage": config["bonus_percentage"],
            "total_raise_usd": total_raise,
            "description": config["description"],
            "tge": {
                "unlock_percentage": config["tge_unlock"],
                "unlock_amount_sbtc": int(config["amount_sbtc"] * config["tge_unlock"]),
                "unlock_amount_mist": tge_unlock_mist
            },
            "vesting": {
                "months": config["vesting_months"],
                "monthly_amount_sbtc": int(config["amount_sbtc"] * (1 - config["tge_unlock"]) / config["vesting_months"])
            }
        }
        accounts.append(account)
    
    return accounts

def generate_tge_snapshot() -> Dict[str, Any]:
    """Generate TGE snapshot."""
    total_circulating = sum([
        TGE_CONFIG["seed_unlock"],
        TGE_CONFIG["private_unlock"],
        TGE_CONFIG["public_unlock"],
        TGE_CONFIG["liquidity_pool"],
        TGE_CONFIG["marketing_airdrops"],
        TGE_CONFIG["team_initial"]
    ])
    
    return {
        "listing_price_usd": TGE_CONFIG["listing_price"],
        "initial_market_cap_usd": total_circulating * TGE_CONFIG["listing_price"],
        "fully_diluted_valuation_usd": TOTAL_SUPPLY * TGE_CONFIG["listing_price"],
        "circulating_supply": {
            "total_sbtc": total_circulating,
            "total_mist": total_circulating * MIST_PER_SBTC,
            "percentage": (total_circulating / TOTAL_SUPPLY) * 100
        },
        "breakdown": {
            "seed_unlock_sbtc": TGE_CONFIG["seed_unlock"],
            "private_unlock_sbtc": TGE_CONFIG["private_unlock"],
            "public_unlock_sbtc": TGE_CONFIG["public_unlock"],
            "liquidity_pool_sbtc": TGE_CONFIG["liquidity_pool"],
            "marketing_airdrops_sbtc": TGE_CONFIG["marketing_airdrops"],
            "team_initial_sbtc": TGE_CONFIG["team_initial"]
        }
    }

def generate_tokenomics_report() -> Dict[str, Any]:
    """Generate complete tokenomics report."""
    allocations = generate_allocation_accounts()
    presale = generate_presale_accounts()
    tge = generate_tge_snapshot()
    
    # Verify total
    total_allocated = sum(a["balance_sbtc"] for a in allocations)
    
    report = {
        "metadata": {
            "generated_at": datetime.now().isoformat(),
            "network": "SilverBitcoin Testnet",
            "version": "1.0.0"
        },
        "summary": {
            "total_supply_sbtc": TOTAL_SUPPLY,
            "total_supply_mist": TOTAL_SUPPLY * MIST_PER_SBTC,
            "decimals": DECIMALS,
            "total_allocated_sbtc": total_allocated,
            "total_allocated_percentage": (total_allocated / TOTAL_SUPPLY) * 100,
            "verification": "PASSED" if total_allocated == TOTAL_SUPPLY else "FAILED"
        },
        "allocations": allocations,
        "presale": presale,
        "tge": tge,
        "emission_schedule": {
            "phase_1_bootstrap": {
                "years": "1-5",
                "annual_emission_sbtc": 50_000_000,
                "fee_burning_percentage": 30,
                "status": "High rewards"
            },
            "phase_2_growth": {
                "years": "6-10",
                "annual_emission_sbtc": 30_000_000,
                "fee_burning_percentage": 50,
                "status": "Balanced"
            },
            "phase_3_maturity": {
                "years": "11-20",
                "annual_emission_sbtc": 10_000_000,
                "fee_burning_percentage": 70,
                "status": "Deflationary"
            },
            "phase_4_perpetual": {
                "years": "20+",
                "annual_emission_sbtc": 0,
                "fee_burning_percentage": 80,
                "status": "Ultra-deflationary"
            }
        }
    }
    
    return report

def main():
    """Generate and output tokenomics report."""
    report = generate_tokenomics_report()
    
    # Output JSON
    print(json.dumps(report, indent=2))
    
    # Also save to file
    with open("tokenomics-report.json", "w") as f:
        json.dump(report, f, indent=2)
    
    print("\n✓ Tokenomics report generated: tokenomics-report.json", file=sys.stderr)
    
    # Print summary
    print("\n=== TOKENOMICS SUMMARY ===", file=sys.stderr)
    print(f"Total Supply: {report['summary']['total_supply_sbtc']:,} SBTC", file=sys.stderr)
    print(f"Total Allocated: {report['summary']['total_allocated_sbtc']:,} SBTC", file=sys.stderr)
    print(f"Verification: {report['summary']['verification']}", file=sys.stderr)
    print("\n=== ALLOCATION BREAKDOWN ===", file=sys.stderr)
    for alloc in report['allocations']:
        print(f"{alloc['category']}: {alloc['balance_sbtc']:,} SBTC ({alloc['percentage']}%)", file=sys.stderr)
    print("\n=== TGE SNAPSHOT ===", file=sys.stderr)
    print(f"Listing Price: ${report['tge']['listing_price_usd']}", file=sys.stderr)
    print(f"Initial Market Cap: ${report['tge']['initial_market_cap_usd']:,.0f}", file=sys.stderr)
    print(f"Fully Diluted Valuation: ${report['tge']['fully_diluted_valuation_usd']:,.0f}", file=sys.stderr)
    print(f"Circulating Supply: {report['tge']['circulating_supply']['total_sbtc']:,} SBTC ({report['tge']['circulating_supply']['percentage']:.1f}%)", file=sys.stderr)

if __name__ == "__main__":
    main()
