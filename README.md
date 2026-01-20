# Blockchain in Rust
This repository showcase my ability to program a blockchain in Rust language

## Functionalities
- **Data Structures:** management of blocks (create, mine), transactions (create), blockchain(status)
- **Proof of Work (Mining):** incrementation of 1 to the block's nonce until the block's hash starts with '0000'.
- **JSON:** automatic backup of blockchain and pool status to local files (`blockchain.json`, `tp.json`, `bp.json`).
- **Pools System:** 
  - `TransactionPool`: stores pending transactions before they are groupe into a block.
  - `BlockPool`: stores created blocks that are waiting to be mined.
- **CLI Interface:** to interact with the blockchain

## Project Stucture
- **Block:** `hash`, `previous_hash`, `nonce`, `timestamp`, `transactions`(Vec), `transaction_counter`, `block_id`.
- **Transaction:** `sender`, `receiver`, `amount`, `transaction_id`.
- **Blockchain:** `blocks` (Vec), `path` (persistence).
- **TransactionPool:** Pending transactions awaiting a block.
- **BlockPool:** Blocks awaiting mining.

## Dependencies
- `serde` & `serde_json`: for data serialization/deserialization
- `sha2`: for SHA-256 hash functions.
- `clap`: for Command Line Interface (CLI).
- `time`: to associate a timestamp to each block.
- `rand`: to generate a random nonce associated to a block.
