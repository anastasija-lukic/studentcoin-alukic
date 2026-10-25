use std::sync::{Arc, Mutex};

use studentcoin::api::{router, AppState};
use studentcoin::blockchain::Blockchain;
use studentcoin::wallet::Wallet;

const ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let node_wallet = Wallet::new();
    let state = AppState {
        blockchain: Arc::new(Mutex::new(Blockchain::new())),
        node_address: node_wallet.address(),
    };

    let listener = tokio::net::TcpListener::bind(ADDRESS)
        .await
        .expect("cannot bind address");

    println!("StudentCoin node running on http://{ADDRESS}");
    println!("Node miner address: {}", node_wallet.address());
    println!("curl.exe http://{ADDRESS}/chain");
    println!("curl.exe http://{ADDRESS}/mine");
    println!("curl.exe http://{ADDRESS}/balance/{}", node_wallet.address());

    axum::serve(listener, router(state)).await.expect("server error");
}
