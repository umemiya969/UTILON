mod consensus;
mod network;
mod state;
mod types;
mod crypto;

use crate::state::NodeState;
use axum::Router;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state: Arc<NodeState> = Arc::new(NodeState {
        jobs: Mutex::new(HashMap::new()),
        votes: Mutex::new(HashMap::new()),
        chain: Mutex::new(Vec::new()),
        finalized: Mutex::new(HashSet::new()),
    });

    let app: Router = network::router(state);

    let addr = "127.0.0.1:9933";
    println!("🚀 UTILON node running at {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
