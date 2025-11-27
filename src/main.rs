mod blockchain;
use crate::{args::{BlockchainArgs, EntityType, TransactionCommand}, blockchain::Transaction};
use blockchain::{Blockchain, TransactionPool};
mod args;
use clap::Parser;

fn main() {
    let mut blockchain = Blockchain::new("./src/blockchain.json".to_string());
    let mut tp: TransactionPool = TransactionPool::new("./src/tp.json".to_string());
    blockchain.clone().genesis_block();
    let args = BlockchainArgs::parse();
    match args.entity {
        EntityType::Transaction { command } => {
            match command {
                TransactionCommand::Create(tx_args) => {
                    if let (Some(sender), Some(receiver), Some(amount)) = (tx_args.sender, tx_args.receiver, tx_args.amount) {
                        let tx = Transaction::new(sender, amount, receiver);
                        tp.add(tx);
                        println!("Transaction ajoutée au pool !")
                    } else {
                        println!("Tous les champs (--sender, --receiver, --amount) sont obligatoires")
                    }
                }
            }
        }
        _ => {}
    }
}