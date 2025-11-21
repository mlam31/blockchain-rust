use clap::Parser;

#[derive(Parser, Debug)]
pub struct BlockchainArgs{
    /// Transaction, Block, TransactionPool, Blockchain
    pub entity_type: String,
    /// Commands
    pub entity_command: String,
    /// Subcommands
    pub entity_subcommands: String
}

// Command for transaction: create, add
// Commands for block: create, mine
// Commands for transaction_pool: status, clear
// Commands for blockchain: status