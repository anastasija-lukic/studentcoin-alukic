use serde::Serialize;

use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug, Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
            mempool: Vec::new(),
        }
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().expect("chain always has the genesis block")
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if !transaction.is_valid() {
            return Err("Invalid transaction signature".to_string());
        }
        if self.mempool.iter().any(|t| t.signature == transaction.signature) {
            return Err("Transaction is already in the mempool".to_string());
        }
        self.mempool.push(transaction);
        Ok(())
    }

    pub fn create_block(&mut self) -> &Block {
        let index = self.last_block().index + 1;
        let previous_hash = self.last_block().hash.clone();
        let transactions = std::mem::take(&mut self.mempool);
        let block = Block::new(index, transactions, previous_hash);
        self.chain.push(block);
        self.last_block()
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.index != previous.index + 1 {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.transactions.iter().any(|t| !t.is_valid()) {
                return false;
            }
        }
        true
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
