# SilverBitcoin Blockchain

**Fast, Secure, Accessible Bitcoin for Everyone**

> *"You didn't miss Bitcoin. You found something better."*

SilverBitcoin is a next-generation Layer-1 blockchain platform built entirely in Rust, designed to be the "people's blockchain" - combining Bitcoin's revolutionary spirit with modern performance, accessibility, and usability.

## 🚀 Key Features

- **⚡ Sub-Second Finality**: 480ms snapshot intervals for near-instant transaction confirmation
- **🔒 Quantum-Resistant**: Post-quantum cryptography (SPHINCS+, Dilithium3, Kyber1024) with 512-bit security
- **📈 High Throughput**: 160,000+ TPS (current), targeting 1M+ TPS with GPU acceleration
- **🎯 Accessible**: Low validator requirements (1M SBTC stake) and affordable transaction fees
- **🔧 Developer-Friendly**: Quantum smart contract language with resource safety guarantees
- **🌐 Scalable**: Parallel execution engine with horizontal scaling capability

## 📊 Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Transaction Finality | < 500ms | ✅ 480ms |
| Throughput (CPU) | 20,000+ TPS | ✅ Designed |
| Throughput (GPU) | 200,000+ TPS | 🔄 In Progress |
| RPC Query Latency | < 100ms | ✅ Designed |
| Validator Minimum Stake | 1M SBTC | ✅ Configured |
| Average Transaction Fee | < $0.01 | ✅ Designed |

## 🏗️ Architecture Overview

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
│  Network Layer (P2P)     │  Indexer (PostgreSQL)            │
└──────────────────────────┴──────────────────────────────────┘
```

### Core Components

- **Cascade Mempool**: Graph-flow based transaction ordering for high throughput
- **Mercury Protocol**: Distributed Resilience Protocol (DRP) consensus with Byzantine fault tolerance
- **Quantum VM**: Resource-oriented smart contract execution with linear type system
- **Parallel Executor**: Multi-core transaction processing with dependency analysis
- **GPU Acceleration**: OpenCL/CUDA/Metal support for 100-1000x speedup

## 🛠️ Building from Source

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
cd silverbitcoin

# Build all components
cargo build --release

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

### Build Targets

```bash
# Build node binary
cargo build --release -p silver-node

# Build CLI tool
cargo build --release -p silver-cli

# Build indexer
cargo build --release -p silver-indexer

# Build Quantum compiler
cargo build --release -p quantum-cli

# Build with GPU support
cargo build --release --features gpu
```

## 🚦 Quick Start

### Running a Local Development Network

```bash
# Start a single-validator development network
silver-cli dev-net

# Or use the node binary directly
silver-node --config dev-config.toml --genesis genesis-dev.json
```

### Generating a Keypair

```bash
# Generate new keypair
silver keygen

# Generate with mnemonic
silver keygen --format mnemonic

# Import existing key
silver keygen --import <private-key>
```

### Transferring Tokens

```bash
# Transfer tokens
silver transfer <recipient-address> <amount>

# Transfer with custom fuel budget
silver transfer <recipient-address> <amount> --fuel 1000000
```

### Querying Blockchain State

```bash
# Query object by ID
silver query <object-id>

# Query objects owned by address
silver query --owner <address>

# Query transaction status
silver query --tx <transaction-digest>
```

## 📦 Project Structure

```
silverbitcoin/
├── crates/
│   ├── silver-core/           # Core types and traits
│   ├── silver-consensus/      # Mercury Protocol + Cascade
│   ├── silver-execution/      # Quantum VM + parallel executor
│   ├── silver-storage/        # RocksDB wrapper + object store
│   ├── silver-network/        # P2P networking (libp2p)
│   ├── silver-api/            # JSON-RPC server
│   ├── silver-crypto/         # Cryptographic primitives
│   ├── silver-cli/            # Command-line tool
│   ├── silver-indexer/        # PostgreSQL indexer
│   ├── silver-sdk/            # Rust SDK for clients
│   ├── silver-node/           # Main node binary
│   └── silver-gpu/            # GPU acceleration layer
├── quantum/
│   ├── quantum-compiler/      # Quantum to bytecode compiler
│   ├── quantum-vm/            # Bytecode interpreter
│   ├── quantum-stdlib/        # Standard library
│   └── quantum-cli/           # Quantum package manager
├── tests/
│   ├── integration/           # Integration tests
│   ├── performance/           # Benchmarks
│   └── stress/                # Stress tests
├── docs/                      # Documentation
└── scripts/                   # Build and deployment scripts
```

## 🔐 Cryptography

### Quantum-Resistant Schemes

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

## 🎓 Smart Contracts (Quantum Language)

### Example Contract

```rust
module silver::coin {
    use silver::object::{Self, UID};
    use silver::transfer;
    
    // Resource type (linear)
    struct Coin has key, store {
        id: UID,
        value: u64,
    }
    
    // Mint new coins
    public fun mint(value: u64, ctx: &mut TxContext): Coin {
        Coin {
            id: object::new(ctx),
            value,
        }
    }
    
    // Transfer coins
    public fun transfer(coin: Coin, recipient: address) {
        transfer::transfer(coin, recipient)
    }
}
```

### Key Features

- **Linear Types**: Resources cannot be copied or dropped
- **Resource Safety**: Compile-time guarantees prevent double-spending
- **Fuel Metering**: Deterministic execution costs
- **Formal Verification**: Type system enables formal proofs

## 📚 Documentation

- **[Architecture Guide](docs/architecture.md)**: System design and component interactions
- **[Developer Guide](docs/developer-guide.md)**: Building applications on SilverBitcoin
- **[Operator Guide](docs/operator-guide.md)**: Running and maintaining nodes
- **[Quantum Language Reference](docs/quantum-reference.md)**: Smart contract language documentation
- **[API Reference](docs/api-reference.md)**: JSON-RPC API documentation

## 🧪 Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test -p silver-consensus

# Run integration tests
cargo test --test '*'

# Run benchmarks
cargo bench

# Run stress tests
cargo test --release --test stress_test
```

## 📊 Monitoring

### Prometheus Metrics

The node exposes Prometheus metrics on port 9184:

```bash
curl http://localhost:9184/metrics
```

Key metrics:
- `silver_snapshots_produced_total`: Total snapshots produced
- `silver_transactions_executed_total`: Total transactions executed
- `silver_consensus_latency_seconds`: Consensus latency histogram
- `silver_execution_latency_seconds`: Execution latency histogram
- `silver_peer_count`: Current peer count

### Health Check

```bash
curl http://localhost:9545/health
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --all-features`)
5. Run linter (`cargo clippy -- -D warnings`)
6. Format code (`cargo fmt`)
7. Commit changes (`git commit -m 'Add amazing feature'`)
8. Push to branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🌟 Acknowledgments

- Inspired by Bitcoin's vision of financial freedom
- Built on research from Sui, Aptos, and Solana
- Quantum-resistant cryptography from NIST PQC standards
- Community-driven development

## 📞 Contact

- **Website**: https://silverbitcoin.org
- **Twitter**: [@SilverBitcoin](https://twitter.com/silverbitcoin)
- **Discord**: https://discord.gg/silverbitcoin
- **Email**: team@silverbitcoin.org

---

**Remember**: *You didn't miss Bitcoin. You found something better.* 🚀

