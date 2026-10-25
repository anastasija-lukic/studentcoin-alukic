use serde::Serialize;

use crate::block::Block;
use crate::transaction::Transaction;

pub const MINING_REWARD: u64 = 50;

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
        if transaction.is_coinbase() {
            return Err("Coinbase transactions cannot be sent to the mempool".to_string());
        }
        if !transaction.is_valid() {
            return Err("Invalid transaction signature".to_string());
        }
        if self.mempool.iter().any(|t| t.signature == transaction.signature) {
            return Err("Transaction is already in the mempool".to_string());
        }
        self.mempool.push(transaction);
        Ok(())
    }

    pub fn mine_block(&mut self, miner: &str) -> &Block {
        let index = self.last_block().index + 1;
        let previous_hash = self.last_block().hash.clone();
        let mut transactions = vec![Transaction::coinbase(miner, MINING_REWARD, index)];
        transactions.append(&mut self.mempool);
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
            if current.hash != current.calculate_hash() || !current.has_valid_proof() {
                return false;
            }
            if !Self::has_valid_transactions(current) {
                return false;
            }
        }
        true
    }

    fn has_valid_transactions(block: &Block) -> bool {
        let Some((coinbase, rest)) = block.transactions.split_first() else {
            return false;
        };
        if !coinbase.is_coinbase() || coinbase.amount != MINING_REWARD {
            return false;
        }
        rest.iter().all(|t| !t.is_coinbase() && t.is_valid())
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
