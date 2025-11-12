use serde::{Serialize, Deserialize};
use serde_json;
use std::sync::atomic::{AtomicU64, Ordering};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use rand;
use std::fs;

static TRANSACTION_COUNTER: AtomicU64 = AtomicU64::new(1);
static BLOCK_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    #[serde(default)]
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
        // clone to calculate hash without hash field
        let mut tmp = self.clone();
        tmp.hash = String::new();
        let data = serde_json::to_string(&tmp).unwrap();
        hasher.update(data);
        let hash_tmp = format!("{:x}", hasher.finalize());
        hash_tmp
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

pub fn initialize_blockchain() {
    let data = fs::read_to_string("./src/blockchain.json").unwrap();
    if data.trim().is_empty() {
        fs::write("./src/blockchain.json", "[]").unwrap();
    }
}

pub fn initialize_genesis_block() {
    // Add Genesis block onto the the blockchain as the 1st block
    let blockchain_data = fs::read_to_string("./src/blockchain.json").unwrap();
    let mut blocks: Vec<Block> = serde_json::from_str(&blockchain_data).unwrap();
    if blocks.is_empty() {
        // Create Genesis block
        let mut transaction_pool: Vec<Transaction> = Vec::new();
        transaction_pool.push(Transaction::new("Bank".to_string(), 10000.0, "Mathieu".to_string()));
        let mut hasher = Sha256::new();
        hasher.update("genesis");
        let genesis_hash = format!("{:x}", hasher.finalize());
        let mut genesis_block = Block::new(genesis_hash, transaction_pool);
        genesis_block.mine_block();
        println!("Genesis block:{:?}", genesis_block);

        blocks.push(genesis_block);
        fs::write("./src/blockchain.json", serde_json::to_string_pretty(&blocks).unwrap()).unwrap();
        println!("Genesis block added to the blockchain !")
    }

}