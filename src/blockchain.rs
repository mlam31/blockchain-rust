use std::sync::atomic::{AtomicU64, Ordering};

static TRANSACTION_COUNTER: AtomicU64 = AtomicU64::new(1);
static BLOCK_COUNTER: AtomicU64 = AtomicU64::new(1);

pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub transaction_counter: u64,
    pub block_id: u64
}

impl Block {
    pub fn new(hash: String, previous_hash: String, nonce: u64, timestamp: u64, transactions: Vec<Transaction>, transaction_counter: u64) -> Self {
        let id = BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst);
        Block{hash, previous_hash, nonce, timestamp, transactions, transaction_counter, block_id: id}
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub amount: f64,
    pub receiver: String,
    pub transaction_id: u64,
}

impl Transaction {
    pub fn new(sender: String, amount: f64, receiver: String) -> Self {
        let id = TRANSACTION_COUNTER.fetch_add(1, Ordering::SeqCst);
        Transaction { sender, amount, receiver, transaction_id: id }
    }
}