mod blockchain;
use crate::{args::{BlockCommand, BlockchainArgs, EntityType, TransactionCommand, TransactionPoolCommand}, blockchain::{Block, Transaction}};
use blockchain::{Blockchain, TransactionPool};
mod args;
use clap::Parser;

fn main() {
    let blockchain = Blockchain::new("./src/blockchain.json".to_string());
    let mut tp: TransactionPool = TransactionPool::new("./src/tp.json".to_string());
    blockchain.clone().genesis_block();
    //tp.add(Transaction::new("Test".to_string(), 1.9, "Test2".to_string()));
    let args = BlockchainArgs::parse();
    match args.entity {
        EntityType::Transaction { command } => {
            match command {
                TransactionCommand::Create(tx_args) => {
                    if let (Some(sender), Some(receiver), Some(amount)) = (tx_args.sender, tx_args.receiver, tx_args.amount) {
                        let tx = Transaction::new(sender, amount, receiver, 0);
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
                        let bk = Block::new(tp.pool, blockchain);
                        todo!("Implement block pool for block not mined yet")
                    }
                }
                BlockCommand::Mine => {
                    todo!("Implement block mining")
                }
                
        
            }
        }
        _ => {}
    }
}