use studentcoin::wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("StudentCoin node");
    println!("New wallet address: {}", wallet.address());
}
