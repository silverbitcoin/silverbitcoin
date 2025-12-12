# SilverBitcoin JSON-RPC API Reference - COMPLETE

**Version:** 2.5.2 - FULL IMPLEMENTATION  
**Network:** SilverBitcoin Mainnet  
**RPC Endpoints:**
- HTTPS: `https://rpc.silverbitcoin.org`
- WebSocket: `wss://rpc.silverbitcoin.org`
- HTTP: `http://127.0.0.1:9000`
- WebSocket: `ws://127.0.0.1:9001`

## Table of Contents

1. [Overview](#overview)
2. [SILVER_ Query Methods (50+ Methods)](#silver-query-methods)
3. [SILVER_ Transaction Methods](#silver-transaction-methods)
4. [SILVER_ Explorer Methods](#silver-explorer-methods)
5. [SILVER_ Validator Methods](#silver-validator-methods)
6. [SILVER_ Token Methods](#silver-token-methods)
7. [SILVER_ Wallet Methods](#silver-wallet-methods)
8. [SILVER_ Admin Methods](#silver-admin-methods)
9. [ETH_ Ethereum Compatibility Methods (40+ Methods)](#eth-ethereum-methods)
10. [Error Codes & Status](#error-codes)
11. [WebSocket Subscriptions](#websocket-subscriptions)
12. [Advanced Examples](#advanced-examples)

---

## Overview

### Server Configuration

- **Max Request Size:** 128KB
- **Max Response Size:** 10MB
- **Max Concurrent Connections:** 1000
- **Rate Limit:** 100 requests/second per IP
- **Batch Request Limit:** 50 requests per batch
- **CORS:** Enabled (configurable)

### Authentication

No authentication required for public endpoints. Admin methods require local connection.

### Response Format

All responses follow JSON-RPC 2.0 specification:

```json
{
  "jsonrpc": "2.0",
  "result": {},
  "id": 1
}
```

---

---

## SILVER_ Query Methods

### silver_getObject

Get an object by ID from the blockchain.

**Parameters:**
- `id` (string, required): Object ID in hex format (0x-prefixed)

**Returns:**
- `id`: Object ID
- `version`: Object version number
- `owner`: Owner address
- `type`: Object type (Coin, Package, Module, etc.)
- `data`: Object data (hex-encoded)
- `previous_transaction`: Previous transaction digest

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "silver_getObject",
  "params": {
    "id": "0x1234567890abcdef..."
  },
  "id": 1
}
```

### silver_getObjectsByOwner

Get all objects owned by an address.

**Parameters:**
- `owner` (string, required): Owner address (0x-prefixed)
- `limit` (number, optional): Max results (default: 50, max: 1000)

**Returns:**
- Array of objects with same structure as `silver_getObject`

### silver_getBalance

Get the balance of an address in SBTC and MIST.

**Parameters:**
- `address` (string, required): Address to query (0x-prefixed)

**Returns:**
- `address`: The queried address
- `balance_mist`: Balance in MIST (1 SBTC = 1,000,000,000 MIST)
- `balance_sbtc`: Balance in SBTC (formatted as decimal string)

**Example Response:**
```json
{
  "address": "0x1234567890abcdef...",
  "balance_mist": 1000000000,
  "balance_sbtc": "1.000000000"
}
```

### silver_getTransaction

Get a transaction by digest.

**Parameters:**
- `digest` (string, required): Transaction digest (hex-encoded)

**Returns:**
- `digest`: Transaction digest
- `status`: Transaction status (Pending, Executed, Failed)
- `fuel_used`: Gas/fuel consumed
- `snapshot`: Optional snapshot data

### silver_getTransactionReceipt

Get the receipt for a transaction.

**Parameters:**
- `digest` (string, required): Transaction digest

**Returns:**
- `transaction_hash`: Transaction hash
- `block_hash`: Block hash containing transaction
- `block_number`: Block number
- `transaction_index`: Index in block
- `from`: Sender address
- `to`: Recipient address (if applicable)
- `cumulative_gas_used`: Total gas used up to this transaction
- `gas_used`: Gas used by this transaction
- `contract_address`: Created contract address (if applicable)
- `logs`: Array of event logs
- `status`: 1 for success, 0 for failure

### silver_getTransactionByHash

Get a transaction by hash.

**Parameters:**
- `hash` (string, required): Transaction hash

**Returns:** Same as `silver_getTransaction`

### silver_getLatestBlockNumber

Get the latest block number.

**Returns:**
- `block_number`: Latest block number

### silver_getBlockByNumber

Get a block by number.

**Parameters:**
- `block_number` (string, required): Block number (decimal or "latest")

**Returns:**
- `number`: Block number
- `hash`: Block hash
- `parent_hash`: Parent block hash
- `timestamp`: Block timestamp (Unix milliseconds)
- `transactions`: Array of transaction hashes in block
- `validator`: Validator address
- `gas_used`: Gas used in block
- `gas_limit`: Gas limit for block

### silver_getGasPrice

Get current gas price.

**Returns:**
- `gas_price`: Current gas price in MIST

### silver_estimateGas

Estimate gas for a transaction.

**Parameters:**
- `commands` (array, optional): Transaction commands

**Returns:**
- `estimated_gas`: Estimated gas amount
- `min_fuel_price`: Minimum fuel price
- `estimated_cost_mist`: Estimated cost in MIST

### silver_getValidators

Get current validator set.

**Returns:** Array of validators with:
- `address`: Validator address
- `name`: Validator name
- `voting_power`: Voting power percentage
- `commission`: Commission rate
- `status`: Validator status (active, inactive, jailed)
- `stake_amount`: Total stake
- `description`: Validator description

### silver_getNetworkStats

Get network statistics.

**Returns:**
- `tps`: Transactions per second
- `total_transactions`: Total transactions processed
- `total_blocks`: Total blocks
- `active_validators`: Number of active validators
- `network_health`: Network health status (healthy, degraded, unhealthy)
- `sample_blocks`: Number of blocks sampled
- `timestamp`: Query timestamp

### silver_getTransactionsByAddress

Get transactions for an address.

**Parameters:**
- `address` (string, required): Address to query
- `limit` (number, optional): Max results (default: 50, max: 500)

**Returns:** Array of transactions

### silver_getTransactionCount

Get transaction count for an address.

**Parameters:**
- `address` (string, required): Address to query

**Returns:**
- `count`: Number of transactions

### silver_getTransactionHistory

Get transaction history for an address.

**Parameters:**
- `address` (string, required): Address to query
- `limit` (number, optional): Max results

**Returns:**
- `address`: Queried address
- `transactions`: Array of transactions
- `total`: Total transaction count

### silver_getCode

Get contract code for an address.

**Parameters:**
- `address` (string, required): Contract address

**Returns:**
- `address`: Contract address
- `code`: Contract bytecode (hex-encoded)

### silver_getEvents

Get events from the blockchain.

**Parameters:**
- `filter` (object, optional): Event filter

**Returns:** Array of events

### silver_queryEvents

Query events with advanced filtering.

**Parameters:**
- `filter` (object, required): Event filter with sender, type, object_type

**Returns:** Array of matching events

### silver_getCheckpoint

Get a checkpoint by number.

**Parameters:**
- `checkpoint_number` (number, required): Checkpoint number

**Returns:** Checkpoint data

### silver_getLatestCheckpoint

Get the latest checkpoint.

**Returns:** Latest checkpoint data

### silver_getAccountInfo

Get detailed account information.

**Parameters:**
- `address` (string, required): Account address

**Returns:**
- `address`: Account address
- `balance`: Account balance
- `object_count`: Number of objects owned
- `description`: Account description (if genesis account)
- `vesting`: Vesting information (if applicable)
- `objects`: Array of owned objects

### silver_getObjectsOwnedByAddress

Get objects owned by an address.

**Parameters:**
- `address` (string, required): Owner address

**Returns:** Array of objects

### silver_getObjectsOwnedByObject

Get objects owned by another object.

**Parameters:**
- `object_id` (string, required): Parent object ID

**Returns:** Array of child objects

---

## Transaction Methods

### silver_submitTransaction

Submit a transaction to the blockchain.

**Parameters:**
- `transaction` (object, required): Transaction data
  - `sender`: Sender address
  - `commands`: Array of commands
  - `gas_budget`: Gas budget
  - `gas_price`: Gas price per unit

**Returns:**
- `digest`: Transaction digest
- `status`: Transaction status
- `timestamp`: Submission timestamp

### silver_dryRunTransaction

Dry run a transaction without submitting.

**Parameters:** Same as `silver_submitTransaction`

**Returns:**
- `effects`: Transaction effects
- `gas_used`: Estimated gas usage
- `status`: Execution status
- `error`: Error message (if failed)

---

## Explorer Methods

### silver_getTokenSupply

Get total token supply information.

**Returns:**
- `total_supply_mist`: Total supply in MIST
- `total_supply_sbtc`: Total supply in SBTC
- `decimals`: Token decimals (9)
- `symbol`: Token symbol (SBTC)
- `hard_cap`: Whether supply is hard-capped
- `allocation`: Token allocation breakdown

### silver_getTokenLargestAccounts

Get accounts with largest token balances.

**Returns:**
- `accounts`: Array of top 100 accounts with:
  - `address`: Account address
  - `balance`: Balance in MIST
  - `decimals`: Token decimals

### silver_getTokenAccountsByOwner

Get token accounts owned by an address.

**Parameters:**
- `owner` (string, required): Owner address

**Returns:**
- `accounts`: Array of token accounts

### silver_getMultipleAccounts

Get information for multiple accounts.

**Parameters:**
- `addresses` (array, required): Array of addresses

**Returns:**
- `accounts`: Array of account information

### silver_getProgramAccounts

Get accounts owned by a program.

**Parameters:**
- `program_id` (string, required): Program ID

**Returns:**
- `accounts`: Array of program accounts

### silver_getSignatureStatuses

Get status of transactions.

**Parameters:**
- `signatures` (array, required): Array of transaction hashes

**Returns:**
- `statuses`: Array of transaction statuses

### silver_getBlockTime

Get block creation time.

**Parameters:**
- `block_number` (number, required): Block number

**Returns:**
- `block_number`: Block number
- `timestamp`: Unix timestamp

### silver_getSlot

Get current slot number.

**Returns:**
- `slot`: Current slot number

### silver_getLeaderSchedule

Get leader schedule.

**Returns:**
- `schedule`: Map of validator IDs to slot assignments

### silver_getClusterNodes

Get cluster node information.

**Returns:** Array of nodes with network information

---

## Validator Methods

### validator_getInfo

Get validator information.

**Parameters:**
- `validator_id` (string, required): Validator ID

**Returns:**
- `validator_id`: Validator ID
- `address`: Validator address
- `stake_amount`: Stake amount
- `commission_rate`: Commission rate
- `participation_rate`: Participation rate
- `uptime_percentage`: Uptime percentage
- `status`: Validator status
- `delegator_count`: Number of delegators
- `total_delegated`: Total delegated amount
- `accumulated_rewards`: Accumulated rewards

### validator_getAllValidators

Get all validators.

**Returns:** Array of validator information

### validator_getDelegationStatus

Get delegation status.

**Parameters:**
- `delegator` (string, required): Delegator address
- `validator_id` (string, required): Validator ID

**Returns:**
- `delegator`: Delegator address
- `validator_id`: Validator ID
- `amount`: Delegated amount
- `accumulated_rewards`: Accumulated rewards
- `delegation_timestamp`: Delegation timestamp
- `status`: Delegation status
- `unbonding_completion_time`: Unbonding completion time (if applicable)

### validator_getDelegations

Get delegations for a delegator.

**Parameters:**
- `delegator` (string, required): Delegator address

**Returns:** Array of delegations

### validator_getValidatorDelegations

Get delegations for a validator.

**Parameters:**
- `validator_id` (string, required): Validator ID

**Returns:** Array of delegations

### validator_claimRewards

Claim rewards.

**Parameters:**
- `delegator` (string, required): Delegator address
- `validator_id` (string, required): Validator ID
- `amount` (number, required): Amount to claim

**Returns:**
- `tx_digest`: Transaction digest
- `amount_claimed`: Amount claimed
- `remaining_rewards`: Remaining rewards
- `status`: Claim status

### validator_submitStake

Submit stake transaction.

**Parameters:**
- `validator` (string, required): Validator address
- `amount` (number, required): Stake amount
- `commission_rate` (number, optional): Commission rate (5-20)

**Returns:**
- `tx_digest`: Transaction digest
- `stake_amount`: Stake amount
- `status`: Submission status

### validator_getRewardHistory

Get reward history.

**Parameters:**
- `delegator` (string, required): Delegator address
- `validator_id` (string, required): Validator ID
- `page` (number, optional): Page number
- `page_size` (number, optional): Page size (max 100)

**Returns:**
- `delegator`: Delegator address
- `validator_id`: Validator ID
- `total_rewards`: Total rewards earned
- `history`: Array of reward history entries
- `total_count`: Total count
- `page`: Current page
- `page_size`: Page size

### validator_getPerformanceMetrics

Get performance metrics.

**Parameters:**
- `validator_id` (string, required): Validator ID

**Returns:**
- `validator_id`: Validator ID
- `participation_rate`: Participation rate
- `uptime_percentage`: Uptime percentage
- `avg_response_time_ms`: Average response time
- `consecutive_failures`: Consecutive failures
- `total_failures`: Total failures
- `last_active`: Last active timestamp

### validator_getActiveAlerts

Get active alerts.

**Parameters:**
- `validator_id` (string, optional): Filter by validator

**Returns:** Array of alerts

### validator_getHealthStatus

Get health status.

**Returns:**
- `status`: Overall status
- `total_validators`: Total validators
- `critical_validators`: Critical validators
- `warning_validators`: Warning validators
- `average_participation`: Average participation
- `average_uptime`: Average uptime
- `average_response_time_ms`: Average response time

---

## Token Methods

### token_createToken

Create a new token.

**Parameters:**
- `creator` (string, required): Creator address
- `name` (string, required): Token name
- `symbol` (string, required): Token symbol
- `decimals` (number, required): Decimal places
- `initial_supply` (string, required): Initial supply
- `fee_paid` (string, required): Creation fee

**Returns:**
- `token_id`: Token ID

### token_transfer

Transfer tokens.

**Parameters:**
- `symbol` (string, required): Token symbol
- `from` (string, required): Sender address
- `to` (string, required): Recipient address
- `amount` (string, required): Transfer amount

**Returns:**
- `tx_hash`: Transaction hash

### token_balanceOf

Get token balance.

**Parameters:**
- `symbol` (string, required): Token symbol
- `account` (string, required): Account address

**Returns:**
- `balance`: Account balance

### token_getMetadata

Get token metadata.

**Parameters:**
- `symbol` (string, required): Token symbol

**Returns:**
- `name`: Token name
- `symbol`: Token symbol
- `decimals`: Decimal places
- `total_supply`: Total supply
- `owner`: Token owner
- `is_paused`: Whether token is paused
- `created_at`: Creation timestamp

### token_approve

Approve token spending.

**Parameters:**
- `symbol` (string, required): Token symbol
- `owner` (string, required): Owner address
- `spender` (string, required): Spender address
- `amount` (string, required): Approval amount

**Returns:**
- `tx_hash`: Transaction hash

### token_allowance

Get approved allowance.

**Parameters:**
- `symbol` (string, required): Token symbol
- `owner` (string, required): Owner address
- `spender` (string, required): Spender address

**Returns:**
- `allowance`: Approved amount

### token_mint

Mint new tokens.

**Parameters:**
- `symbol` (string, required): Token symbol
- `minter` (string, required): Minter address
- `to` (string, required): Recipient address
- `amount` (string, required): Mint amount

**Returns:**
- `tx_hash`: Transaction hash

### token_burn

Burn tokens.

**Parameters:**
- `symbol` (string, required): Token symbol
- `burner` (string, required): Burner address
- `from` (string, required): Account to burn from
- `amount` (string, required): Burn amount

**Returns:**
- `tx_hash`: Transaction hash

### token_totalSupply

Get total token supply.

**Parameters:**
- `symbol` (string, required): Token symbol

**Returns:**
- `total_supply`: Total supply

### token_listTokens

List all tokens.

**Returns:** Array of token metadata

---

## Wallet Methods

### wallet_generateWallet

Generate a new wallet with mnemonic.

**Parameters:**
- `word_count` (number, optional): Mnemonic word count (12, 15, 18, 21, 24)

**Returns:**
- `mnemonic`: BIP39 mnemonic phrase
- `address`: Generated address
- `public_key`: Public key
- `derivation_path`: Derivation path used
- `word_count`: Word count

### wallet_importWallet

Import wallet from mnemonic.

**Parameters:**
- `mnemonic` (string, required): BIP39 mnemonic phrase
- `derivation_path` (string, optional): BIP44 derivation path

**Returns:**
- `address`: Imported address
- `public_key`: Public key
- `derivation_path`: Derivation path used

### wallet_deriveAddresses

Derive multiple addresses from mnemonic.

**Parameters:**
- `mnemonic` (string, required): BIP39 mnemonic phrase
- `count` (number, optional): Number of addresses (default: 10)
- `start_index` (number, optional): Starting index (default: 0)

**Returns:**
- `addresses`: Array of derived addresses
- `count`: Number of addresses derived

### wallet_validateMnemonic

Validate a mnemonic phrase.

**Parameters:**
- `mnemonic` (string, required): Mnemonic phrase to validate

**Returns:**
- `is_valid`: Whether mnemonic is valid
- `mnemonic`: The mnemonic phrase

### wallet_importPrivateKey

Import wallet from private key.

**Parameters:**
- `private_key` (string, required): Private key (hex format)

**Returns:**
- `address`: Imported address
- `public_key`: Public key
- `import_method`: Import method used

### wallet_importKeystore

Import wallet from Geth/MetaMask keystore.

**Parameters:**
- `keystore_json` (string, required): Keystore JSON
- `password` (string, required): Keystore password

**Returns:**
- `address`: Imported address
- `public_key`: Public key
- `import_method`: Import method

### wallet_encryptWallet

Encrypt and store wallet.

**Parameters:**
- `private_key` (string, required): Private key (hex)
- `password` (string, required): Encryption password
- `name` (string, optional): Wallet name
- `argon2_params` (object, optional): Argon2 parameters

**Returns:**
- `address`: Wallet address
- `encrypted_data`: Encrypted wallet data
- `name`: Wallet name
- `created_at`: Creation timestamp

### wallet_decryptWallet

Decrypt stored wallet.

**Parameters:**
- `encrypted_data` (string, required): Encrypted wallet data
- `password` (string, required): Decryption password

**Returns:**
- `private_key`: Decrypted private key
- `private_key_bytes`: Key size in bytes

### wallet_listWallets

List all stored wallets.

**Returns:**
- `wallets`: Array of wallet information
- `count`: Number of wallets

### wallet_getWallet

Get wallet by address.

**Parameters:**
- `address` (string, required): Wallet address

**Returns:**
- `address`: Wallet address
- `name`: Wallet name
- `created_at`: Creation timestamp
- `is_default`: Whether wallet is default

### wallet_deleteWallet

Delete a wallet.

**Parameters:**
- `address` (string, required): Wallet address

**Returns:**
- `address`: Deleted wallet address
- `deleted`: Deletion status

### wallet_renameWallet

Rename a wallet.

**Parameters:**
- `address` (string, required): Wallet address
- `new_name` (string, required): New wallet name

**Returns:**
- `address`: Wallet address
- `name`: New wallet name

### wallet_setDefaultWallet

Set default wallet.

**Parameters:**
- `address` (string, required): Wallet address

**Returns:**
- `address`: Wallet address
- `is_default`: Default status

### wallet_getDefaultWallet

Get default wallet.

**Returns:**
- `address`: Default wallet address
- `name`: Wallet name
- `created_at`: Creation timestamp

### wallet_deriveAccounts

Derive multiple accounts from mnemonic.

**Parameters:**
- `mnemonic` (string, required): BIP39 mnemonic phrase
- `count` (number, optional): Number of accounts (default: 5)
- `start_index` (number, optional): Starting index (default: 0)

**Returns:**
- `accounts`: Array of derived accounts
- `count`: Number of accounts

### wallet_getDerivationPaths

Get supported derivation paths.

**Returns:**
- `paths`: Array of supported paths with descriptions

---

## Admin Methods

### admin_addPeer

Add a peer to the network.

**Parameters:**
- `peer_id` (string, required): Peer identifier
- `address` (string, required): Peer IP address
- `port` (number, required): Peer port

**Returns:**
- `success`: Operation success
- `message`: Success message

### admin_removePeer

Remove a peer from the network.

**Parameters:**
- `peer_id` (string, required): Peer identifier

**Returns:**
- `success`: Operation success
- `message`: Success message

### admin_peers

Get list of connected peers.

**Returns:** Array of peer information

### admin_peerCount

Get number of connected peers.

**Returns:**
- `count`: Number of peers

### admin_nodeInfo

Get node information.

**Returns:**
- `id`: Node ID
- `name`: Node name
- `enode`: enode URL
- `ip`: Node IP
- `ports`: Port information
- `protocols`: Protocol information

### admin_startRPC

Start HTTP RPC server.

**Returns:**
- `success`: Operation success
- `message`: Success message
- `address`: RPC server address
- `status`: Server status

### admin_stopRPC

Stop HTTP RPC server.

**Returns:**
- `success`: Operation success
- `message`: Success message

### admin_startWS

Start WebSocket RPC server.

**Returns:**
- `success`: Operation success
- `message`: Success message
- `address`: WebSocket server address

### admin_stopWS

Stop WebSocket RPC server.

**Returns:**
- `success`: Operation success
- `message`: Success message

### admin_datadir

Get data directory.

**Returns:** Data directory path

### admin_setSolc

Set Solidity compiler path.

**Parameters:**
- `path` (string, required): Compiler path

**Returns:**
- `success`: Operation success
- `message`: Success message

---

## Ethereum Compatibility

The API provides Ethereum-compatible methods for interoperability:

### eth_getBalance

Get account balance (Ethereum compatible).

**Parameters:**
- `address` (string): Account address
- `block` (string): Block number or "latest"

**Returns:** Balance in wei

### eth_getCode

Get contract code (Ethereum compatible).

**Parameters:**
- `address` (string): Contract address
- `block` (string): Block number or "latest"

**Returns:** Contract bytecode

### eth_call

Execute a contract call (Ethereum compatible).

**Parameters:**
- `to` (string): Contract address
- `data` (string): Call data
- `block` (string): Block number or "latest"

**Returns:** Call result

### eth_sendTransaction

Send a transaction (Ethereum compatible).

**Parameters:**
- `from` (string): Sender address
- `to` (string): Recipient address
- `value` (string): Value in wei
- `data` (string): Transaction data
- `gas` (string): Gas limit
- `gasPrice` (string): Gas price

**Returns:** Transaction hash

### eth_getTransactionReceipt

Get transaction receipt (Ethereum compatible).

**Parameters:**
- `hash` (string): Transaction hash

**Returns:** Transaction receipt

### eth_blockNumber

Get latest block number (Ethereum compatible).

**Returns:** Block number in hex

### eth_getBlockByNumber

Get block by number (Ethereum compatible).

**Parameters:**
- `block` (string): Block number or "latest"
- `full` (boolean): Include full transaction data

**Returns:** Block data

---

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON was received |
| -32600 | Invalid request | Request is not valid JSON-RPC |
| -32601 | Method not found | Method does not exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal server error |
| -32000 | Rate limit exceeded | Too many requests from this IP |
| -32001 | Object not found | Requested object does not exist |
| -32002 | Transaction not found | Requested transaction does not exist |
| -32003 | Block not found | Requested block does not exist |
| -32004 | Receipt not found | Transaction receipt not found |
| -32005 | Invalid address | Address format is invalid |

---

## WebSocket Subscriptions

### silver_subscribeEvents

Subscribe to blockchain events.

**Parameters:**
- `filter` (object, optional): Event filter
  - `sender`: Filter by sender address
  - `event_type`: Filter by event type
  - `object_type`: Filter by object type
  - `object_id`: Filter by object ID
  - `transaction_digest`: Filter by transaction

**Returns:**
- `subscription_id`: Subscription ID

**Event Format:**
```json
{
  "subscription_id": "0x...",
  "event_id": 123,
  "transaction_digest": "0x...",
  "event_type": "ObjectCreated",
  "sender": "0x...",
  "object_id": "0x...",
  "object_type": "Coin",
  "data": "0x...",
  "timestamp": 1234567890000
}
```

### silver_unsubscribe

Unsubscribe from events.

**Parameters:**
- `subscription_id` (string, required): Subscription ID to cancel

**Returns:**
- `message`: Unsubscribe confirmation

---

## Rate Limiting

- **Limit:** 100 requests per second per IP
- **Burst:** Up to 100 requests allowed
- **Refill Rate:** 100 tokens per second
- **Response:** 429 Too Many Requests when exceeded

---

## Best Practices

1. **Batch Requests:** Use batch requests for multiple queries (max 50)
2. **Caching:** Cache frequently accessed data (blocks, validators)
3. **Error Handling:** Implement exponential backoff for retries
4. **WebSocket:** Use WebSocket for real-time event subscriptions
5. **Gas Estimation:** Always estimate gas before submitting transactions
6. **Address Format:** Always use 0x-prefixed hex format for addresses

---

## Examples

### Get Account Balance

```bash
curl -X POST http://127.0.0.1:9000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "silver_getBalance",
    "params": {
      "address": "0x1234567890abcdef..."
    },
    "id": 1
  }'
```

### Submit Transaction

```bash
curl -X POST http://127.0.0.1:9000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "silver_submitTransaction",
    "params": {
      "transaction": {
        "sender": "0x...",
        "commands": [...],
        "gas_budget": 50000,
        "gas_price": 1000
      }
    },
    "id": 1
  }'
```

### Batch Request

```bash
curl -X POST http://127.0.0.1:9000 \
  -H "Content-Type: application/json" \
  -d '[
    {
      "jsonrpc": "2.0",
      "method": "silver_getLatestBlockNumber",
      "id": 1
    },
    {
      "jsonrpc": "2.0",
      "method": "silver_getValidators",
      "id": 2
    }
  ]'
```

### WebSocket Subscription

```javascript
const ws = new WebSocket('ws://127.0.0.1:9001');

ws.onopen = () => {
  ws.send(JSON.stringify({
    method: "silver_subscribeEvents",
    filter: {
      event_type: "ObjectCreated"
    }
  }));
};

ws.onmessage = (event) => {
  const notification = JSON.parse(event.data);
  console.log('Event:', notification);
};
```

---

**Last Updated:** December 2025  
**Maintained By:** SilverBitcoin Development Team
