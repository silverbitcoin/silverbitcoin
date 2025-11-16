# 🪙 SilverBitcoin Blockchain

<div align="center">

![SilverBitcoin Logo](logo.png)

## 🌟 Our Story: The Second Chance

**You didn't miss Bitcoin. You found something better.**

When Bitcoin emerged in 2009, it promised financial freedom for everyone. But as its value soared to $100,000+, that promise became a distant dream for most people. The very thing that made Bitcoin valuable—its scarcity—also made it inaccessible.

**SilverBitcoin was born from a simple question:** *What if we could capture Bitcoin's revolutionary spirit, but make it accessible, fast, and practical for everyday use?*

### 💫 Why "Silver" Bitcoin?

Just as silver has always been "the people's precious metal"—affordable, practical, and valuable—SilverBitcoin is designed to be the blockchain for everyone. While Bitcoin became digital gold, locked away in vaults, SilverBitcoin flows freely, powering real transactions, real applications, and real opportunities.

### 🚀 Our Mission

We're not trying to replace Bitcoin. We're completing its vision:
- **Speed**: Sub-second finality (480ms) vs Bitcoin's 60 minutes
- **Accessibility**: Low entry barriers for validators and users
- **Usability**: Full smart contract support for DeFi, NFTs, and real-world applications
- **Scalability**: **160,000+ TPS** currently, targeting 1M+ TPS with GPU acceleration

[![License: CC BY 4.0](https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by/4.0/)
[![Rust Version](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org)
[![Security](https://img.shields.io/badge/Security-Quantum--Resistant-success.svg)](docs/security.md)
[![Consensus](https://img.shields.io/badge/Consensus-Cascade+Mercury-blue.svg)](.kiro/specs/silverbitcoin-blockchain/design.md)

[Website](https://silverbitcoin.org) • [Explorer](https://blockchain.silverbitcoin.org) • [Whitepaper](WHITEPAPER.md) • [Telegram](https://t.me/SilverBitcoinLabs)

</div>

---

## 🎯 Key Features

SilverBitcoin is a next-generation Layer-1 blockchain platform built entirely in **Rust**, featuring:

### ⚡ Ultra-Fast Performance
- **480ms Finality**: Sub-second transaction confirmation
- **160,000+ TPS**: Current throughput with horizontal scaling
- **1M+ TPS Target**: With GPU acceleration and optimizations
- **Parallel Execution**: Multi-threaded transaction processing

### 🔒 Quantum-Resistant Security
- **SPHINCS+**: Post-quantum hash-based signatures
- **Dilithium3**: NIST Level 3 lattice-based signatures
- **Kyber1024**: Post-quantum key encapsulation
- **Blake3-512**: 512-bit hashing for quantum resistance
- **Hybrid Mode**: Classical + post-quantum for transition period

### 🌊 Cascade + Mercury Protocol
- **DAG-Based Mempool**: Parallel batch creation by validators
- **Deterministic Ordering**: Flow graph traversal without leader election
- **Byzantine Fault Tolerance**: Tolerates up to 1/3 malicious validators
- **Distributed Resilience**: No single point of failure

### 🧠 Quantum VM
- **Move-Inspired**: Resource-oriented programming model
- **Linear Types**: Compile-time safety guarantees
- **Fuel Metering**: Deterministic execution costs
- **Parallel Execution**: Independent transactions run concurrently

### 💎 Object-Centric Model
- **Owned Objects**: Single-owner, fast execution
- **Shared Objects**: Multi-transaction access with consensus
- **Immutable Objects**: Read-only, no consensus needed
- **Flexible Attributes**: Dynamic key-value pairs

### 🚀 GPU Acceleration
- **OpenCL/CUDA/Metal**: Cross-platform GPU support
- **100-1000x Speedup**: Batch signature verification
- **10-100x Speedup**: Hash computation and execution
- **Auto-Detection**: Automatic GPU selection and CPU fallback

### 🔐 Recursive zk-SNARKs (Mina-Inspired)
- **Constant-Size Blockchain**: ~100 MB regardless of history
- **Compression**: constant size ~100 MB 
- **Instant Light Client Sync**: Seconds instead of days
- **Mobile-Friendly Verification**: Full verification on mobile devices
- **Proof Incentives**: Validators earn 10 SBTC per proof

---

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.75 or later
- **System Dependencies**:
  - RocksDB development libraries
  - OpenSSL development libraries
  - Protocol Buffers compiler
  - (Optional) CUDA toolkit for GPU acceleration
  - (Optional) OpenCL drivers for GPU acceleration

### Installation

```bash
# Clone the repository
git clone https://github.com/silverbitcoin/silverbitcoin.git
cd silverbitcoin/silverbitcoin-blockchain

# Build all components
cargo build --release

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

### Running a Node

```bash
# Start a validator node
cargo run --release --bin silver-node -- \
  --config config/validator.toml \
  --genesis config/genesis.json \
  --validator-keys keys/validator.json

# Start a full node (non-validator)
cargo run --release --bin silver-node -- \
  --config config/fullnode.toml \
  --genesis config/genesis.json
```

### Development Network

```bash
# Start a local single-validator network
silver-cli dev-net

# Or use the node binary directly
silver-node --config dev-config.toml --genesis genesis-dev.json
```

---

## 📁 Project Structure

```
SilverBitcoin/
├── silverbitcoin-blockchain/  # Rust blockchain implementation
│   ├── crates/
│   │   ├── silver-core/       # Core types and utilities
│   │   ├── silver-crypto/     # Quantum-resistant cryptography (SPHINCS+, Dilithium3, Kyber1024)
│   │   ├── silver-storage/    # RocksDB storage layer
│   │   ├── silver-archive-chain/ # Archive Chain (3 TPS historical record)
│   │   ├── silver-api/        # JSON-RPC API gateway
│   │   ├── silver-consensus/  # Cascade + Mercury Protocol consensus
│   │   ├── silver-execution/  # Quantum VM execution engine
│   │   ├── silver-network/    # libp2p P2P networking
│   │   ├── silver-node/       # Main node binary
│   │   ├── silver-cli/        # Command-line interface
│   │   ├── silver-sdk/        # Rust SDK for developers
│   │   ├── silver-gpu/        # GPU acceleration (OpenCL/CUDA/Metal)
│   │   ├── silver-zksnark/    # Recursive zk-SNARKs (Mina-style constant-size blockchain)
│   │   ├── silver-coordinator/# Transaction coordinator
│   │   └── silver-light-client/ # Light client with query system
│   ├── quantum/               # Quantum language implementation
│   │   ├── quantum-compiler/  # Quantum to bytecode compiler
│   │   ├── quantum-vm/        # Bytecode interpreter
│   │   ├── quantum-stdlib/    # Standard library
│   │   └── quantum-cli/       # Quantum package manager
│   ├── docs/                  # Technical documentation
│   ├── benchmarks/            # Performance benchmarks
│   ├── docker/                # Docker deployment files
│   └── scripts/               # Build and deployment scripts
├── website/                   # Official website (silverbitcoin.org)
├── blockchain-explorer/       # Blockchain explorer UI
├── staking-dashboard/         # Staking platform
├── validator-dashboard/       # Validator management panel
├── silver-wallet/             # Wallet application
├── whitepaper/                # Technical whitepaper
└── docs/                      # Project-wide documentation
```

---

## 🌐 Network Information

### Mainnet Configuration

| Parameter | Value |
|-----------|-------|
| **Network Name** | SilverBitcoin Mainnet |
| **JSON-RPC URL** | `https://rpc.silverbitcoin.org/` |
| **WebSocket URL** | `wss://ws.silverbitcoin.org/` |
| **Block Explorer** | https://blockchain.silverbitcoin.org/ |
| **Snapshot Interval** | 480ms (sub-second finality) |
| **Consensus** | Cascade + Mercury Protocol |
| **Currency Symbol** | SBTC |
| **Currency Decimals** | 9 (1 SBTC = 1,000,000,000 MIST) |
| **Maximum Supply** | 1,000,000,000 SBTC (HARD CAP) |
| **Genesis Supply** | 1,000,000,000 SBTC (all minted at genesis) |
| **Presale Allocation** | 100M SBTC (10% of total supply) |
| **Circulating (TGE)** | 60M SBTC (6% - includes partial presale unlock + liquidity + marketing) |
| **Long-term Circulating** | ~200-300M SBTC (deflationary from Year 11+) |

### Connect with Rust SDK

```rust
use silver_sdk::{SilverClient, types::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to SilverBitcoin
    let client = SilverClient::new("https://rpc.silverbitcoin.org/").await?;
    
    // Query object
    let object = client.get_object(object_id).await?;
    println!("Object: {:?}", object);
    
    // Submit transaction
    let tx = TransactionBuilder::new()
        .transfer(recipient, amount)
        .build()?;
    
    let digest = client.submit_transaction(tx).await?;
    println!("Transaction: {}", hex::encode(digest));
    
    Ok(())
}
```

### CLI Commands

```bash
# Generate keypair
silver-cli keygen

# Transfer tokens
silver-cli transfer \
  --to <recipient-address> \
  --amount 1000000000 \
  --fuel-budget 50000000

# Query object
silver-cli object <object-id>

# Call Quantum function
silver-cli call \
  --package <package-id> \
  --module <module-name> \
  --function <function-name> \
  --args <arg1> <arg2>
```

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     SilverBitcoin Node                      │
├─────────────────────────────────────────────────────────────┤
│  API Gateway  │  CLI Tool  │  Metrics (Prometheus)          │
│  (JSON-RPC)   │            │                                │
├───────────────┴────────────┴────────────────────────────────┤
│           Transaction Coordinator                           │
├──────────────────────────┬──────────────────────────────────┤
│  Consensus Engine        │  Execution Engine                │
│  (Mercury Protocol)      │  (Quantum VM)                    │
│  - Cascade Mempool       │  - Parallel Executor             │
│  - Flow Graph            │  - Fuel Metering                 │
├──────────────────────────┴──────────────────────────────────┤
│              Object Store (RocksDB)                         │
├──────────────────────────┬──────────────────────────────────┤
│  Network Layer (P2P)     │  Archive Chain (Historical)      │
└──────────────────────────┴──────────────────────────────────┘
```

### Cascade + Mercury Protocol

**Cascade Mempool (Phase 1):**
- Worker-based batch creation (500 tx/batch, 512KB max)
- Flow graph with cryptographic links (Blake3-512)
- Parallel batch propagation (<50ms target)
- Certificate collection (2/3+ stake signatures)

**Mercury Protocol (Phase 2):**
- Deterministic flow graph traversal
- Topological sort with hash-based tie-breaking
- Ordered transaction execution
- Snapshot creation (480ms intervals)
- Sub-second finality

**Key Properties:**
- **Byzantine Fault Tolerance**: Tolerates up to 1/3 malicious validators
- **Parallel Processing**: Independent transactions execute concurrently
- **Energy Efficient**: No wasteful mining
- **Scalable**: 160,000+ TPS currently, 1M+ TPS target

---

## 💼 Use Cases

### 💰 DeFi Applications
- Decentralized exchanges with sub-second finality
- Lending protocols with high throughput
- Yield farming with low fees
- Derivatives trading with fast settlement

### 🎮 Gaming & NFTs
- GameFi with instant transactions
- NFT marketplaces with high volume
- Metaverse economies with real-time interactions
- Digital collectibles with provable ownership

### 🏢 Enterprise Solutions
- Supply chain tracking with immutable records
- Identity management with privacy
- Payment systems with instant settlement
- Asset tokenization with regulatory compliance

---

## 🛠️ Development

### Quantum Language

SilverBitcoin uses **Quantum**, a Move-inspired smart contract language with:
- Linear types and borrow checking
- Object-centric programming model
- Resource safety guarantees
- Bytecode compilation

### Example Quantum Module

```rust
module silver::coin {
    use silver::object::{Self, UID};
    use silver::transfer;
    use silver::tx_context::{Self, TxContext};

    struct Coin has key, store {
        id: UID,
        balance: u64,
    }

    public fun mint(amount: u64, ctx: &mut TxContext): Coin {
        Coin {
            id: object::new(ctx),
            balance: amount,
        }
    }

    public fun transfer(coin: Coin, recipient: address) {
        transfer::transfer(coin, recipient)
    }

    public fun balance(coin: &Coin): u64 {
        coin.balance
    }
}
```

### Create a Quantum Package

```bash
# Create new package
quantum new my_package
cd my_package

# Build package
quantum build

# Run tests
quantum test

# Publish to blockchain
quantum publish --fuel-budget 100000000
```

---

## 📊 Performance Metrics

### Network Statistics

- **Snapshot Interval**: 480ms (sub-second finality)
- **Throughput**: 160,000+ TPS (current), 1,000,000 TPS (target with GPU)
- **Batch Size**: 500 transactions or 512KB (whichever comes first)
- **Batch Propagation**: <50ms target
- **Finality**: Sub-second (after snapshot)
- **Consensus**: Cascade + Mercury Protocol
- **Byzantine Tolerance**: Up to 1/3 malicious validators

### Fuel Costs

SilverBitcoin uses **fuel metering** for transaction costs:

```
Simple Transfer:     ~1,000 fuel units
Object Creation:     ~5,000 fuel units
Quantum Function:    ~10,000 fuel units (varies)
Module Publish:      ~100,000 fuel units

Minimum Fuel Price:  1,000 MIST per fuel unit
```

**Example Transaction Cost:**
```
10,000 fuel × 1,000 MIST = 10,000,000 MIST = 0.01 SBTC
```

## 🔐 Security

### Quantum-Resistant Cryptography

| Scheme | Type | Security | Size | Speed | Use Case |
|--------|------|----------|------|-------|----------|
| **SPHINCS+** | Hash-based | 256-bit PQ | 49 KB | 1.5ms | Validators |
| **Dilithium3** | Lattice | 192-bit PQ | 3.3 KB | 0.5ms | Users |
| **Secp512r1** | Elliptic Curve | 256-bit Classical | 132 B | 0.3ms | Legacy |
| **Hybrid** | Combined | 256-bit PQ | 52 KB | 2ms | Maximum Security |

### Key Features

- **512-bit Security**: All addresses and hashes use 512-bit Blake3 for quantum resistance
- **Hybrid Mode**: Combines classical + post-quantum for transition period
- **Key Encryption**: XChaCha20-Poly1305 + Kyber1024 + Argon2id
- **HD Wallets**: BIP32/BIP39 extended to 512-bit derivation

### Important Security Notes

- 🔒 **Never commit private keys** (`keys/*.key`, `keys/*.json`)
- 🔒 **Secure validator keys** (protocol, network, worker keys)
- 🔒 **Use SSL/TLS for public RPC** (Nginx reverse proxy with Let's Encrypt)
- 🔒 **Restrict RPC access** to trusted IPs only
- 🔒 **Enable key encryption** (XChaCha20-Poly1305 with strong passphrase)
- 🔒 **Backup keys securely** (encrypted, offline storage)
- 🔒 **Keep software updated** (security patches and upgrades)

---

## 💰 Tokenomics

### Hard Cap: 1 Billion SBTC (FIXED)

**Supply Model:**
- **Maximum Supply:** 1,000,000,000 SBTC (NEVER EXCEEDS)
- **Genesis Allocation:** All 1B minted at genesis
- **Emission:** 20-year schedule from Validator Rewards Pool
- **Fee Burning:** 30% → 80% (increasing over time)
- **Long-term:** Deflationary from Year 11 onwards

**Emission Schedule:**

| Phase | Years | Annual Emission | Fee Burning | Status |
|-------|-------|-----------------|-------------|--------|
| Bootstrap | 1-5 | 50M SBTC | 30% | High rewards |
| Growth | 6-10 | 30M SBTC | 50% | Balanced |
| Maturity | 11-20 | 10M SBTC | 70% | Deflationary |
| Perpetual | 20+ | 0 SBTC | 80% | Ultra-deflationary |

**Token Distribution:**
- Community Reserve: 30% (300M SBTC)
- Validator Rewards Pool: 25% (250M SBTC)
- Ecosystem Fund: 15% (150M SBTC)
- Presale/Public: 10% (100M SBTC) - Avalanche-inspired
- Team & Advisors: 10% (100M SBTC) - 4 year vest
- Foundation: 5% (50M SBTC)
- Early Investors: 5% (50M SBTC) - 2 year vest

**Presale Pricing :**
- Seed Round: $0.33 per SBTC (20M SBTC, 30% bonus) → $6.6M
  - TGE Unlock: 20% (4M SBTC)
  - Vesting: 80% over 12 months
- Private Sale: $0.50 per SBTC (30M SBTC, 20% bonus) → $15M
  - TGE Unlock: 30% (9M SBTC)
  - Vesting: 70% over 8 months
- Public Presale: $0.50 per SBTC (50M SBTC, 10% bonus) → $25M
  - TGE Unlock: 50% (25M SBTC)
  - Vesting: 50% over 4 months
- **Total Presale: 100M SBTC (10% of supply)**
- **Total Raise: $46.6M**

**TGE (Token Generation Event):**
- Listing Price: $3.00 per SBTC (6x from public)
- Initial Circulating: 60M SBTC (6%)
  - Presale Unlock: 38M SBTC (4M + 9M + 25M)
  - Liquidity Pool: 10M SBTC
  - Marketing/Airdrops: 5M SBTC
  - Team Initial: 7M SBTC
- Initial Market Cap: $180M (60M × $3.00)
- Fully Diluted Valuation: $3B (1B × $3.00)

**Key Features:**
- ✅ **Hard Cap:** 1B SBTC maximum, never more
- ✅ **Deflationary:** Fee burning creates scarcity
- ✅ **Sustainable:** 20-year emission schedule
- ✅ **Ultra-Scarce:** ~200-300M circulating long-term
- ✅ **High APY:** 10-30%+ as supply shrinks

**Detailed Tokenomics:** [TOKENOMICS.md](TOKENOMICS.md)

---

## 📚 Documentation


### Developer Guides

- **[Quantum Language Guide](docs/quantum-language.md)** - Smart contract development
- **[Rust SDK Documentation](silverbitcoin-blockchain/crates/silver-sdk/README.md)** - SDK usage and examples
- **[CLI Reference](silverbitcoin-blockchain/crates/silver-cli/README.md)** - Command-line tool documentation
- **[API Reference](silverbitcoin-blockchain/crates/silver-api/README.md)** - JSON-RPC API documentation

### Validator Guides

- **[Validator Setup](docs/validator-setup.md)** - How to run a validator
- **[Key Management](docs/key-management.md)** - Secure key handling
- **[Monitoring](docs/monitoring.md)** - Node monitoring and metrics

### Technical Papers

- **[Whitepaper](WHITEPAPER.md)** - SilverBitcoin technical whitepaper
- **[Cascade Protocol](docs/cascade-protocol.md)** - DAG-based mempool
- **[Mercury Protocol](docs/mercury-protocol.md)** - Deterministic ordering
- **[Quantum VM](docs/quantum-vm.md)** - Bytecode interpreter design

---

## 📈 Roadmap

### Q4 2025 (Current)
- ✅ Production Mainnet Launch (November 2025)
- ✅ Cascade + Mercury Protocol Consensus
- ✅ Quantum VM Execution Engine
- ✅ Post-Quantum Cryptography
- 🔄 DeFi Ecosystem Growth
- 🔄 Developer Tools & SDKs

### Q1-Q2 2026
- 🚀 Enhanced Governance Features
- 🚀 Cross-Chain Bridge Development
- 🚀 DeFi Protocol Partnerships
- 🚀 Mobile Wallet Launch
- 🚀 Enterprise Integrations

### 2026+ Research & Development

**🌟Performance Enhancements:**
- 📋 **GPU Acceleration** - CUDA/OpenCL/Metal support (100-1000× speedup for proof generation)
- 📋 **Parallel Processing** - Multi-threaded transaction validation (4-8× improvement)
- 📋 **State Optimization** - Advanced pruning and compression (60-80% storage reduction)
- 📋 **Target**: 1,000,000 TPS by 2027

**Scaling Solutions:**
- 📋 **Layer 2 Rollups** - Optimistic and ZK-Rollups (100-1000× compression)
- 📋 **Horizontal Sharding** - Multiple parallel chains (10× per shard)

**Security & Privacy:**
- 📋 **Advanced Privacy** - Zero-knowledge proofs and confidential transactions
- 📋 **Cross-Chain Bridges** - Secure interoperability with major blockchains

---

## 🤝 Community

### Get Involved

- **Telegram**: [SilverBitcoin Labs](https://t.me/SilverBitcoinLabs)
- **Twitter**: [@SilverBitcoinLabs](https://x.com/silverbitcoinlabs)
- **GitHub**: Contribute to the codebase
- **Medium**: Technical articles

### Governance

- Submit improvement proposals
- Vote on network changes
- Become a validator
- Join ambassador program

---

## 🆘 Support

### Community Support
- 💬 Telegram: Real-time help
- 🐛 GitHub Issues: Bug reports
- 📧 Email: info@silverbitcoin.org

### Professional Support
- Enterprise support packages
- Custom development services
- Training and certification

---

## 📄 License

Creative Commons Attribution 4.0 International License (CC BY 4.0) - see [LICENSE](LICENSE) file for details.

---

## ⚠️ Disclaimer

Blockchain technology involves inherent risks. Users should:
- Understand the technology before using
- Never invest more than they can afford to lose
- Keep private keys secure and backed up
- Verify all transactions before confirming

---

<div align="center">

**Built with ❤️ by the SilverBitcoin Foundation**

⭐ Star us on GitHub — it helps!

[Website](https://silverbitcoin.org) • [Explorer](https://blockchain.silverbitcoin.org) • [Telegram](https://t.me/SilverBitcoinLabs)

*Empowering the decentralized future, one block at a time.*

</div>

---

*Last updated: November 2025*
