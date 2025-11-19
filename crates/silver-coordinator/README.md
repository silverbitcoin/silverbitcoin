# Silver Coordinator

Transaction coordinator for SilverBitcoin blockchain that manages the complete transaction lifecycle from submission to finalization.

## Features

### Transaction Submission Handler
- Validates incoming transactions (structure, signatures, expiration)
- Verifies fuel payment object exists and has sufficient balance
- Returns transaction digest within 100ms (requirement 7.3)
- Supports both regular and sponsored transactions

### Transaction Lifecycle Management
- Tracks transaction status: Pending → Executing → Executed/Failed/Expired
- Coordinates between consensus and execution engines
- Handles transaction expiration automatically
- Provides statistics and monitoring capabilities
- Supports up to 100,000 active transactions (configurable)

### Transaction Sponsorship
- Validates sponsor signatures (requirement 21.2)
- Verifies sponsor owns fuel payment object (requirement 21.3)
- Validates sponsor has sufficient balance (requirement 21.4)
- Processes fuel refunds to sponsor after execution (requirement 21.5)
- Ensures sponsor is different from sender (requirement 21.1)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│           Transaction Coordinator                        │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────────────────────────────────────┐       │
│  │  Submission Handler                          │       │
│  │  - Validate transaction structure            │       │
│  │  - Verify signatures                         │       │
│  │  - Check expiration                          │       │
│  │  - Validate fuel payment                     │       │
│  └──────────────┬───────────────────────────────┘       │
│                 │                                         │
│  ┌──────────────▼───────────────────────────────┐       │
│  │  Lifecycle Manager                           │       │
│  │  - Track transaction status                  │       │
│  │  - Handle expiration                         │       │
│  │  - Provide statistics                        │       │
│  └──────────────┬───────────────────────────────┘       │
│                 │                                         │
│  ┌──────────────▼───────────────────────────────┐       │
│  │  Sponsorship Validator                       │       │
│  │  - Validate sponsor signatures               │       │
│  │  - Verify sponsor balance                    │       │
│  │  - Process fuel refunds                      │       │
│  └──────────────────────────────────────────────┘       │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Usage

```rust
use silver_coordinator::{TransactionCoordinator, CoordinatorConfig};
use tokio::sync::mpsc;

// Create channels for consensus and execution
let (consensus_tx, consensus_rx) = mpsc::unbounded_channel();
let (execution_tx, execution_rx) = mpsc::unbounded_channel();

// Create coordinator
let config = CoordinatorConfig::default();
let coordinator = TransactionCoordinator::new(
    config,
    object_store,
    consensus_tx,
    execution_rx,
);

// Submit a transaction
let digest = coordinator.submit_transaction(transaction).await?;

// Check transaction status
let status = coordinator.get_transaction_status(&digest);

// Start background tasks
let coordinator = Arc::new(coordinator);
coordinator.start_background_tasks();
```

## Requirements Satisfied

- **Requirement 7.3**: Transaction submission with validation and digest return
- **Requirement 3.1**: Transaction signature verification
- **Requirement 3.2**: Transaction lifecycle tracking
- **Requirement 21.1**: Sponsor must be different from sender
- **Requirement 21.2**: Sponsor signature validation
- **Requirement 21.3**: Sponsor fuel object ownership validation
- **Requirement 21.4**: Sponsor balance validation
- **Requirement 21.5**: Fuel refund to sponsor

## Testing

Run tests with:
```bash
cargo test --package silver-coordinator
```

All 15 tests pass, covering:
- Transaction submission and validation
- Lifecycle state transitions
- Expiration handling
- Sponsorship validation
- Statistics tracking
- Capacity limits

## Performance

- Transaction submission: < 100ms (requirement 7.3)
- Supports 100,000+ active transactions
- Automatic cleanup of expired transactions
- Efficient in-memory tracking with DashMap

