use std::env;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use studentcoin::api::{router, AppState};
use studentcoin::blockchain::Blockchain;
use studentcoin::cli;
use studentcoin::wallet::Wallet;

const ADDRESS: &str = "127.0.0.1:3000";

const USAGE: &str = "usage:
  studentcoin                                            start the node
  studentcoin wallet                                     create a new wallet
  studentcoin sign <secret> <receiver> <amount> <nonce>  sign a transaction";

async fn run_node() -> Result<(), String> {
    let node_wallet = Wallet::new();
    let state = AppState {
        blockchain: Arc::new(Mutex::new(Blockchain::new())),
        node_address: node_wallet.address(),
    };

    let listener = tokio::net::TcpListener::bind(ADDRESS)
        .await
        .map_err(|e| format!("cannot bind {ADDRESS}: {e}"))?;

    println!("StudentCoin node running on http://{ADDRESS}");
    println!("Node miner address: {}", node_wallet.address());
    println!("curl.exe http://{ADDRESS}/chain");
    println!("curl.exe http://{ADDRESS}/mine");
    println!("curl.exe http://{ADDRESS}/balance/{}", node_wallet.address());

    axum::serve(listener, router(state))
        .await
        .map_err(|e| format!("server error: {e}"))
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let result = match args.as_slice() {
        [] => run_node().await,
        [mode] if mode == "wallet" => {
            cli::new_wallet();
            Ok(())
        }
        [mode, secret, receiver, amount, nonce] if mode == "sign" => {
            cli::sign(secret, receiver, amount, nonce).map(|path| {
                println!("saved to {}", path.display());
            })
        }
        _ => Err(USAGE.to_string()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}
