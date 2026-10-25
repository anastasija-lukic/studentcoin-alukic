use studentcoin::transaction::Transaction;
use studentcoin::wallet::Wallet;

fn main() {
    let alice = Wallet::new();
    let bob = Wallet::new();
    println!("StudentCoin node");
    println!("Alice: {}", alice.address());
    println!("Bob:   {}", bob.address());

    let transaction = Transaction::new(&alice, &bob.address(), 10);
    println!("Transaction valid: {}", transaction.is_valid());
}
