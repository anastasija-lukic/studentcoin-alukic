use serde::Serialize;
use std::collections::HashMap;

use crate::transaction::Transaction;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

pub type Accounts = HashMap<String, Account>;

pub fn apply(accounts: &mut Accounts, transaction: &Transaction) -> Result<(), String> {
    if !transaction.is_coinbase() {
        let sender = accounts.get(&transaction.sender).cloned().unwrap_or_default();
        if transaction.nonce != sender.nonce {
            return Err(format!("Wrong nonce, expected {}", sender.nonce));
        }
        let new_balance = sender
            .balance
            .checked_sub(transaction.amount)
            .ok_or("Insufficient funds")?;
        let sender = accounts.entry(transaction.sender.clone()).or_default();
        sender.balance = new_balance;
        sender.nonce += 1;
    }

    let receiver = accounts.entry(transaction.receiver.clone()).or_default();
    receiver.balance = receiver
        .balance
        .checked_add(transaction.amount)
        .ok_or("Balance overflow")?;
    Ok(())
}
