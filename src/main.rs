mod blockchain;
use blockchain::{Block, Transaction, initialize_blockchain};
use sha2::{Sha256, Digest};
use crate::blockchain::initialize_genesis_block;

fn main() {
    initialize_blockchain();
    initialize_genesis_block();
}

fn simulate_transactions() -> Vec<Transaction>{
    let mut transaction_pool: Vec<Transaction> = Vec::new();
    let t1 = Transaction::new("Sender1".to_string(), 1.0, "Receiver1".to_string());
    let t2 = Transaction::new("Sender2".to_string(), 1.0, "Receiver2".to_string());
    let t3 = Transaction::new("Sender3".to_string(), 1.0, "Receiver3".to_string());
    let t4 = Transaction::new("Sender4".to_string(), 1.0, "Receiver4".to_string());
    let t5 = Transaction::new("Sender5".to_string(), 1.0, "Receiver5".to_string());
    transaction_pool.push(t1);
    transaction_pool.push(t2);
    transaction_pool.push(t3);
    transaction_pool.push(t4);
    transaction_pool.push(t5);
    transaction_pool
}
