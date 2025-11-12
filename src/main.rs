mod blockchain;
use blockchain::{Block, Transaction, initialize_blockchain};
use sha2::{Sha256, Digest};
use crate::blockchain::initialize_genesis_block;

fn main() {
    initialize_blockchain();
    initialize_genesis_block();
}

