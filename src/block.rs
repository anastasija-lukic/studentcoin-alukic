use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hash::to_hex;
use crate::transaction::Transaction;

pub const DIFFICULTY: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        Block::mined(index, current_timestamp(), transactions, previous_hash)
    }

    pub fn genesis() -> Self {
        Block::mined(0, 0, Vec::new(), "0".repeat(64))
    }

    fn mined(index: u64, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        let (nonce, hash) = block.mine();
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    pub fn mine(&self) -> (u64, String) {
        let prefix = "0".repeat(DIFFICULTY);
        let transactions = self.transactions_json();
        for nonce in 0u64.. {
            let hash = self.hash_with(&transactions, nonce);
            if hash.starts_with(&prefix) {
                return (nonce, hash);
            }
        }
        unreachable!("the search always returns a nonce")
    }

    pub fn calculate_hash(&self) -> String {
        self.hash_with(&self.transactions_json(), self.nonce)
    }

    pub fn has_valid_proof(&self) -> bool {
        self.hash.starts_with(&"0".repeat(DIFFICULTY))
    }

    fn transactions_json(&self) -> String {
        serde_json::to_string(&self.transactions).unwrap_or_default()
    }

    fn hash_with(&self, transactions: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(transactions.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(nonce.to_le_bytes());
        to_hex(&hasher.finalize())
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
