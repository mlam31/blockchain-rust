use serde::{Serialize, Deserialize};
use serde_json;
use std::{sync::atomic::{AtomicU64, Ordering}};
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub amount: f64,
    pub receiver: String,
    pub transaction_id: u64,
}
#[derive(Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
#[derive(Debug, Clone)]
pub struct TransactionPool {
    pub transaction_pool: Vec<Transaction>
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, blockchain: Blockchain) -> Self {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let transaction_counter = transactions.len() as u64;
        let id = BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst);
        let nonce = rand::random::<u64>();
        let previous_hash = blockchain.get_previous_hash();
        
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

    pub fn mine_block(&mut self, mut blockchain: Blockchain) {
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
        blockchain.blocks.push(self.clone());
        blockchain.save().unwrap();
        println!("\nBlock added to the blockchain: \n{:#?} ! \n", self)
    }
}


impl Transaction {
    pub fn new(sender: String, amount: f64, receiver: String) -> Self {
        let id = TRANSACTION_COUNTER.fetch_add(1, Ordering::SeqCst);
        Transaction {sender, amount, receiver, transaction_id: id}
    }
}

pub fn initialize_blockchain() {
    let data = fs::read_to_string("./src/blockchain.json").unwrap();
    if data.trim().is_empty() {
        fs::write("./src/blockchain.json", "[]").unwrap();
    }
}


impl Blockchain {
    pub fn new() -> Self {
        initialize_blockchain();
        let data = fs::read_to_string("./src/blockchain.json").unwrap();
        let blocks: Vec<Block> = serde_json::from_str(&data).unwrap();
        Blockchain{blocks}
    }

    pub fn genesis_block(self) {
        if self.blocks.is_empty(){
            let mut genesis_tp = TransactionPool::new();
            let genesis_transaction = Transaction::new("Bank".to_string(), 10000.0, "Mathieu".to_string());
            genesis_tp.add(genesis_transaction);
            let mut genesis_block = Block::new(genesis_tp.transaction_pool, self.clone());
            genesis_block.mine_block(self);
            println!("Genesis block: \n{:#?} \n", genesis_block);
            println!("Genesis block added to the blockchain !\n")
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error>{
        let path = "./src/blockchain.json";
        fs::write(path, serde_json::to_string_pretty(&self.blocks).unwrap())
    }

    pub fn get_previous_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update("genesis");
        let genesis_hash = format!("{:x}", hasher.finalize());
        if let Some(previous_block) = self.blocks.last().cloned() {
            let previous_hash = previous_block.hash;
            previous_hash
        } else {
            genesis_hash
        }
    }
}

impl TransactionPool {
    pub fn new() -> Self {
        let transaction_pool: Vec<Transaction> = Vec::new();
        TransactionPool { transaction_pool }
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction)
    }

    pub fn clear_pool(&mut self) {
        self.transaction_pool.clear()
    }
}