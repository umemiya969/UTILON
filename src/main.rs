mod consensus;
mod network;
mod state;
mod types;

use axum::Server;
use state::NodeState;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    let state = Arc::new(NodeState {
        jobs: Mutex::new(HashMap::new()),
        votes: Mutex::new(HashMap::new()),
        finalized: Mutex::new(HashSet::new()),
        chain: Mutex::new(Vec::new()),
        balances: Mutex::new(HashMap::new()),
    });

    let app = network::router(state);

    let port = std::env::var("PORT").unwrap_or("9933".into());
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

    println!("🚀 UTILON running at {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
