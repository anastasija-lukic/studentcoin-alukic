use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transaction::{current_timestamp, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: current_timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn genesis() -> Self {
        let mut block = Block {
            index: 0,
            timestamp: 0,
            transactions: Vec::new(),
            previous_hash: "0".repeat(64),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transactions = serde_json::to_string(&self.transactions).unwrap_or_default();
        let data = format!(
            "{}{}{}{}",
            self.index, self.timestamp, transactions, self.previous_hash
        );
        hex::encode(Sha256::digest(data.as_bytes()))
    }
}
