mod blockchain;
use blockchain::{Block, Transaction, Blockchain, initialize_blockchain};
use sha2::{Sha256, Digest};
use crate::blockchain::initialize_genesis_block;

fn main() {
    initialize_blockchain();
    let mut blockchain = Blockchain::new();
    initialize_genesis_block(blockchain.blocks);
}
