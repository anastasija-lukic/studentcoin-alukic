use studentcoin::blockchain::Blockchain;
use studentcoin::transaction::Transaction;
use studentcoin::wallet::Wallet;

fn main() {
    let alice = Wallet::new();
    let bob = Wallet::new();
    let mut blockchain = Blockchain::new();
    println!("StudentCoin node");

    let block = blockchain.mine_block(&alice.address());
    println!("Mined block {} nonce {} hash {}", block.index, block.nonce, block.hash);

    let nonce = blockchain.next_nonce(&alice.address());
    let transaction = Transaction::new(&alice, &bob.address(), 10, nonce);
    match blockchain.add_transaction(transaction) {
        Ok(()) => println!("Transaction of 10 added to mempool"),
        Err(e) => println!("Transaction rejected: {}", e),
    }

    let nonce = blockchain.next_nonce(&alice.address());
    let transaction = Transaction::new(&alice, &bob.address(), 100, nonce);
    match blockchain.add_transaction(transaction) {
        Ok(()) => println!("Transaction of 100 added to mempool"),
        Err(e) => println!("Transaction of 100 rejected: {}", e),
    }

    let block = blockchain.mine_block(&bob.address());
    println!("Mined block {} nonce {} hash {}", block.index, block.nonce, block.hash);
    println!("Alice balance: {}", blockchain.balance(&alice.address()));
    println!("Bob balance: {}", blockchain.balance(&bob.address()));
    println!("Chain valid: {}", blockchain.is_valid());
}
