mod blockchain;
use crate::{args::BlockchainArgs, blockchain::Transaction};
use blockchain::{Blockchain, TransactionPool};
mod args;
use clap::Parser;

fn main() {
    let mut blockchain = Blockchain::new("./src/blockchain.json".to_string());
    let mut tp: TransactionPool = TransactionPool::new("./src/tp.json".to_string());
    blockchain.clone().genesis_block();
    //let args = BlockchainArgs::parse();
    let t1 = Transaction::new("alice".to_string(), 50.0, "bob".to_string(), tp);
}