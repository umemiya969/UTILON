mod types;
mod state;
mod consensus;
mod storage;
mod p2p;
mod network;

use std::sync::{Arc, Mutex};
use axum::Router;
use tokio::net::TcpListener;
use crate::state::NodeState;

#[tokio::main]
async fn main() {
    let mut state = NodeState::new();
    state.chain = storage::load_chain();

    let shared = Arc::new(Mutex::new(state));
    let app: Router = network::router(shared);

    let listener = TcpListener::bind("127.0.0.1:9933").await.unwrap();
    println!("🧱 PoUC node running on 9933");

    axum::serve(listener, app).await.unwrap();
}
