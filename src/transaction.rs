use serde::{Deserialize, Serialize};

use crate::hash::sha256;
use crate::wallet::{address_from_public_key, verify_signature, Wallet};

pub const COINBASE_SENDER: &str = "COINBASE";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub public_key: String,
    pub signature: String,
}

impl Transaction {
    pub fn new(wallet: &Wallet, receiver: &str, amount: u64, nonce: u64) -> Self {
        let mut transaction = Transaction {
            sender: wallet.address(),
            receiver: receiver.to_string(),
            amount,
            nonce,
            public_key: wallet.public_key_hex(),
            signature: String::new(),
        };
        transaction.signature = wallet.sign(&transaction.hash());
        transaction
    }

    pub fn coinbase(miner: &str, reward: u64, block_index: u64) -> Self {
        Transaction {
            sender: COINBASE_SENDER.to_string(),
            receiver: miner.to_string(),
            amount: reward,
            nonce: block_index,
            public_key: String::new(),
            signature: String::new(),
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.sender == COINBASE_SENDER
    }

    pub fn hash(&self) -> [u8; 32] {
        let data = format!(
            "{}:{}:{}:{}:{}",
            self.sender, self.receiver, self.amount, self.nonce, self.public_key
        );
        sha256(data.as_bytes())
    }

    pub fn is_valid(&self) -> bool {
        if self.amount == 0 || self.sender == self.receiver {
            return false;
        }
        let public_key = match hex::decode(&self.public_key) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        if address_from_public_key(&public_key) != self.sender {
            return false;
        }
        verify_signature(&self.public_key, &self.hash(), &self.signature)
    }
}
