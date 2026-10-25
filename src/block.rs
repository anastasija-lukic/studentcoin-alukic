use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hash::to_hex;
use crate::transaction::Transaction;

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
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(transactions.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        to_hex(&hasher.finalize())
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
