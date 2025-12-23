mod blockchain;
use crate::{args::{BlockCommand, BlockPoolCommand, BlockchainArgs, EntityType, TransactionCommand, TransactionPoolCommand}, blockchain::{Block, BlockPool, Transaction}};
use blockchain::{Blockchain, TransactionPool};
mod args;
use clap::Parser;

fn main() {
    let blockchain = Blockchain::new("./src/blockchain.json".to_string());
    let mut tp: TransactionPool = TransactionPool::new("./src/tp.json".to_string());
    let mut bp: BlockPool = BlockPool::new("./src/bp.json".to_string());
    blockchain.clone().genesis_block();




    let args = BlockchainArgs::parse();
    match args.entity {
        EntityType::Transaction { command } => {
            match command {
                TransactionCommand::Create(tx_args) => {
                    if let (Some(sender), Some(receiver), Some(amount)) = (tx_args.sender, tx_args.receiver, tx_args.amount) {
                        let tx = Transaction::new(sender, amount, receiver, tp.next_transaction_id);
                        tp.add(tx);
                        println!("Transaction added to the pool !")
                    } else {
                        println!("Tous les champs (--sender, --receiver, --amount) sont obligatoires")
                    }
                }
            }
        }
        EntityType::TransactionPool  { command  }=> {
            match command {
                    TransactionPoolCommand::Show => {
                        if tp.pool.is_empty(){
                            println!("Transaction pool is empty")
                        } else {
                            println!("------- Transactions in the pool -------");
                            for (i, tx) in tp.pool.iter().enumerate() {
                                println!("[{}] {:?}", i+1, tx)
                            }
                        }
                    }
                    TransactionPoolCommand::Clear => {
                        if tp.pool.is_empty(){
                            println!("The pool is already empty")
                        } else {
                            tp.pool.clear();
                            tp.save().unwrap();
                            println!("The pool is cleared")
                        }
                    }
                    TransactionPoolCommand::Status => {

                    }
            }
        }
        EntityType::Block { command } => {
            match command {
                BlockCommand::Create => {
                    if tp.pool.is_empty(){
                        println!("Block can't be created if transaction pool is empty")
                    } else {
                        let bk = Block::new(tp.clone().pool, blockchain, bp.next_block_id);
                        bp.add(bk);
                        tp.pool.clear();
                        tp.save().unwrap();
                        println!("Block added to the pool");  
                    }
                }
                BlockCommand::Mine => {
                    todo!("Implement block mining")
                } 
            }
        }
        EntityType::BlockPool { command } => {
            match command {
                BlockPoolCommand::Show => {
                    if bp.pool.is_empty(){
                        println!("The block pool is empty")
                    } else {
                        for (i, bp) in bp.pool.iter().enumerate(){
                            println!("[{}] {:?}", i+1, bp)
                        }
                    }
                }
            }
        }
        _ => {}
    }
}