mod blockchain;
use blockchain::{Block, Transaction, Blockchain, initialize_blockchain};
use sha2::{Sha256, Digest};
use crate::blockchain::initialize_genesis_block;

fn main() {
    initialize_blockchain();
    let mut blockchain = Blockchain::new();
    initialize_genesis_block(blockchain.clone());
    
    //println!("{:?}", blockchain.blocks);
    let last_block = blockchain.blocks.last().unwrap();
    println!("{:#?}", last_block);
    loop {
        let transactions = std::mem::take(&mut blockchain.mem_pool);
        if transactions.len() == 10 {
            let previous_block = blockchain.blocks.last().unwrap().clone();
            let previous_hash = previous_block.hash;
            Block::new(previous_hash, transactions);
            
        }
    }
}
