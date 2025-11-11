use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicU64, Ordering};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

static TRANSACTION_COUNTER: AtomicU64 = AtomicU64::new(1);
static BLOCK_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    #[serde(skip_serializing)]
    pub hash: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub transaction_counter: u64,
    pub block_id: u64
}

impl Block {
    pub fn new(previous_hash: String, nonce: u64, transactions: Vec<Transaction>) -> Self {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let transaction_counter = transactions.len() as u64;
        let id = BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst);
        
        let mut block = Block {
            hash: String::new(),
            previous_hash,
            nonce,
            timestamp,
            transactions,
            transaction_counter,
            block_id: id
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = serde_json::to_string(self).unwrap();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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