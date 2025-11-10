pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub transaction_counter: u64,
    pub block_id: u64
}

pub struct BlockGenerator {
    pub next_id: u128
}

impl BlockGenerator {
    pub fn new() -> Self {
        BlockGenerator { next_id: 1 }
    }

    pub fn create_block(&mut self, hash: String, previous_hash: String, nonce: u64, timestamp: u64, transactions: Vec<Transaction>, transaction_counter: u64) -> Block {
       let b = Block{hash, previous_hash, nonce, timestamp, transactions, transaction_counter, block_id: self.next_id};
       self.next_id += 1;
       b
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub amount: f64,
    pub receiver: String,
    pub transaction_id: u64,
}

pub struct TransactionGenerator {
    pub next_id: u64
}

impl TransactionGenerator {
    pub fn new() -> Self{
        TransactionGenerator { next_id: 1 }
    }

    pub fn create_transaction(&mut self, sender: String, amount: f64, receiver: String) -> Transaction {
        let t = Transaction { sender, amount, receiver, transaction_id: self.next_id };
        self.next_id += 1;
        t
    }
}