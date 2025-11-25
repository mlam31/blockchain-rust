mod blockchain;
use crate::args::BlockchainArgs;
use blockchain::{Blockchain, TransactionPool};
mod args;
use clap::Parser;

fn main() {
    let mut blockchain = Blockchain::new("./src/blockchain.json".to_string());
    let mut tp: TransactionPool = TransactionPool::new();
    blockchain.clone().genesis_block();
    //let args = BlockchainArgs::parse();
}
