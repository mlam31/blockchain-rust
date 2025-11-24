use clap::{Parser, Subcommand};
use crate::blockchain::{Block, Blockchain, Transaction, TransactionPool};

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
    TransactionPool
}

#[derive(Subcommand, Debug)]
pub enum TransactionCommand {
    Create(TransactionArgs)
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
    pub amout: Option<f64>,
}

impl TransactionArgs {}

// Command for transaction: create
// Commands for block: create, mine
// Commands for transaction_pool: status, clear
// Commands for blockchain: status