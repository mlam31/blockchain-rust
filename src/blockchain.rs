use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicU64, Ordering};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use rand;

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
    pub fn new(previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let transaction_counter = transactions.len() as u64;
        let id = BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst);
        let nonce = rand::random::<u64>();
        
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

    pub fn mine_block(&mut self) {
        let mut new_nonce = 0;
        loop {
            self.nonce = new_nonce;
            self.hash = self.calculate_hash();
            if self.hash.starts_with("0000"){
                break;
            }
            else {
                new_nonce = new_nonce.wrapping_add(1);
                println!("Trying new nonce: {}", new_nonce)
            }
        }
        println!("New hash: {}\nBlock mined ! ", self.hash)
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