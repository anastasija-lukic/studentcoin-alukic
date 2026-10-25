use std::fs;
use std::path::PathBuf;

use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub const TRANSACTION_FILE: &str = "podaci/transakcija.json";

pub fn new_wallet() {
    let wallet = Wallet::new();
    println!("secret:     {}", wallet.secret_hex());
    println!("public key: {}", wallet.public_key_hex());
    println!("address:    {}", wallet.address());
}

pub fn sign(secret: &str, receiver: &str, amount: &str, nonce: &str) -> Result<PathBuf, String> {
    let wallet = Wallet::from_secret_hex(secret).ok_or("secret must be 64 hex characters")?;
    let amount: u64 = amount.parse().map_err(|_| "amount must be a whole number")?;
    let nonce: u64 = nonce.parse().map_err(|_| "nonce must be a whole number")?;

    let transaction = Transaction::new(&wallet, receiver, amount, nonce);
    let json = serde_json::to_string_pretty(&transaction).map_err(|e| e.to_string())?;

    let path = PathBuf::from(TRANSACTION_FILE);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    }
    fs::write(&path, &json).map_err(|e| format!("cannot write {}: {e}", path.display()))?;

    println!("{json}");
    Ok(path)
}
