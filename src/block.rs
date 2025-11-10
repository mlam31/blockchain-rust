mod transaction;
use transaction::{Transaction};

pub struct Block {
    hash: String,
    previous_hash: String,
    timestamp: u64,
    transactions: Vec<Transaction>,
    transaction_counter: i32,
}