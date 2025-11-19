mod blockchain;
use blockchain::{Transaction, Blockchain};
use crate::blockchain::{TransactionPool,};

fn main() {
    let blockchain = Blockchain::new();
    let mut tp: TransactionPool = TransactionPool::new();
    blockchain.genesis_block();
    let t1 = Transaction::new("Test".to_string(), 1.0, "Test".to_string());
    tp.add(t1);
    println!("{:?}", tp)
}
