use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::wallet::{verify_signature, Wallet};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(wallet: &Wallet, receiver: &str, amount: u64) -> Self {
        let mut transaction = Transaction {
            sender: wallet.address(),
            receiver: receiver.to_string(),
            amount,
            timestamp: current_timestamp(),
            signature: String::new(),
        };
        transaction.signature = wallet.sign(transaction.payload().as_bytes());
        transaction
    }

    pub fn payload(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.sender, self.receiver, self.amount, self.timestamp
        )
    }

    pub fn is_valid(&self) -> bool {
        if self.amount == 0 || self.sender == self.receiver {
            return false;
        }
        verify_signature(&self.sender, self.payload().as_bytes(), &self.signature)
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
