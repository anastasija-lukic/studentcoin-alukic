use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub node_address: String,
}

#[derive(Serialize)]
struct ChainResponse {
    length: usize,
    chain: Vec<Block>,
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Deserialize)]
struct MineQuery {
    miner: Option<String>,
}

#[derive(Serialize)]
struct MineResponse {
    message: String,
    block: Block,
}

#[derive(Serialize)]
struct BalanceResponse {
    address: String,
    balance: u64,
    next_nonce: u64,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/chain", get(chain))
        .route("/transactions/new", post(new_transaction))
        .route("/mine", get(mine))
        .route("/balance/:address", get(balance))
        .with_state(state)
}

async fn chain(State(state): State<AppState>) -> Json<ChainResponse> {
    let blockchain = state.blockchain.lock().expect("lock");
    Json(ChainResponse {
        length: blockchain.chain.len(),
        chain: blockchain.chain.clone(),
    })
}

async fn new_transaction(
    State(state): State<AppState>,
    Json(transaction): Json<Transaction>,
) -> (StatusCode, Json<MessageResponse>) {
    let mut blockchain = state.blockchain.lock().expect("lock");
    match blockchain.add_transaction(transaction) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(MessageResponse {
                message: "Transaction added to mempool".to_string(),
            }),
        ),
        Err(e) => (StatusCode::BAD_REQUEST, Json(MessageResponse { message: e })),
    }
}

async fn mine(State(state): State<AppState>, Query(query): Query<MineQuery>) -> Json<MineResponse> {
    let miner = query.miner.unwrap_or(state.node_address.clone());
    let mut blockchain = state.blockchain.lock().expect("lock");
    let block = blockchain.mine_block(&miner).clone();
    Json(MineResponse {
        message: format!("Block {} mined", block.index),
        block,
    })
}

async fn balance(State(state): State<AppState>, Path(address): Path<String>) -> Json<BalanceResponse> {
    let blockchain = state.blockchain.lock().expect("lock");
    Json(BalanceResponse {
        balance: blockchain.balance(&address),
        next_nonce: blockchain.next_nonce(&address),
        address,
    })
}
