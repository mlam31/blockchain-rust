use clap::{Parser, Subcommand};
use crate::{args, blockchain::{Block, Blockchain, Transaction, TransactionPool}};

#[derive(Parser, Debug)]
pub struct BlockchainArgs{
    #[command(subcommand)]
    pub entity: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Block,
    Blockchain,
    Transaction {
        #[command(subcommand)]
        command: TransactionCommand,
    },
    TransactionPool {
        #[command(subcommand)]
        command: TransactionPoolCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum TransactionCommand {
    Create(TransactionArgs)
}

#[derive(Subcommand, Debug)]
pub enum  TransactionPoolCommand {
    Show,
    Status,
    Clear,
}

#[derive(Parser, Debug)]
pub struct TransactionArgs {
    /// The one who sends the amount
    #[arg(short, long)]
    pub sender: Option<String>,
    /// The one who receives the amount
    #[arg(short, long)]
    pub receiver: Option<String>,
    /// The amount 
    #[arg(short, long)]
    pub amount: Option<f64>,
    /// The transaction pool
    #[arg(short, long)]
    pub tp: Option<String>
}

// Command for transaction: create
// Commands for block: create, mine
// Commands for transaction_pool: status, clear
// Commands for blockchain: status