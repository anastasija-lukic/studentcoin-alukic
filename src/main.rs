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

    let transaction = Transaction::new(&alice, &bob.address(), 10, 0);
    match blockchain.add_transaction(transaction) {
        Ok(()) => println!("Transaction added to mempool"),
        Err(e) => println!("Transaction rejected: {}", e),
    }

    let block = blockchain.mine_block(&bob.address());
    println!("Mined block {} nonce {} hash {}", block.index, block.nonce, block.hash);
    println!("Chain length: {}", blockchain.chain.len());
    println!("Chain valid: {}", blockchain.is_valid());
}
