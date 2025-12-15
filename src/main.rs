mod blockchain;
use crate::{args::{BlockchainArgs, EntityType, TransactionCommand, TransactionPoolCommand}, blockchain::Transaction};
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
                        let tx = Transaction::new(sender, amount, receiver);
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
                        println!("------- Transactions in the pool -------");
                        if tp.pool.is_empty(){
                            println!("Transaction pool is empty")
                        } else {
                            for (i, tx) in tp.pool.iter().enumerate() {
                                println!("[{}] {:?}", i+1, tx)
                            }
                        }
                    }
                    TransactionPoolCommand::Clear => {

                    }
                    TransactionPoolCommand::Status => {

                    }
            }
        }
        _ => {}
    }
}