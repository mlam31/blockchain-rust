use std::{sync::mpsc::Sender, time::{SystemTime, UNIX_EPOCH}};
mod blockchain;
use blockchain::{Block, Transaction, TransactionGenerator};

fn main() {
    let mut transaction_pool: Vec<Transaction> = Vec::new();
    simulate_transaction(&mut transaction_pool);
    println!("{:?}", transaction_pool)
}

fn simulate_transaction(transaction_pool: &mut Vec<Transaction>){
    let mut generator = TransactionGenerator::new();
    let transaction1 = generator.create_transaction("Sender1".to_string(), 1.0, "Receiver1".to_string());
    let transaction2 = generator.create_transaction("Sender2".to_string(), 1.0, "Receiver2".to_string());
    let transaction3 = generator.create_transaction("Sender3".to_string(), 1.0, "Receiver3".to_string());
    let transaction4 = generator.create_transaction("Sender4".to_string(), 1.0, "Receiver4".to_string());
    let transaction5 = generator.create_transaction("Sender5".to_string(), 1.0, "Receiver5".to_string());
    transaction_pool.push(transaction1);
    transaction_pool.push(transaction2);
    transaction_pool.push(transaction3);
    transaction_pool.push(transaction4);
    transaction_pool.push(transaction5);
}