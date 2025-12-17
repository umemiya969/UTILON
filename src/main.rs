mod types;
mod state;
mod consensus;
mod network;
mod validate;
mod storage;


use std::sync::{Arc, Mutex};
use state::NodeState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    use crate::storage::load_chain;

    let mut state = NodeState::new();
    state.chain = load_chain();
    println!("📦 Loaded chain with {} blocks", state.chain.len());

    use crate::storage::save_chain;

    save_chain(&state.chain);

    let state = Arc::new(Mutex::new(state));
    let app = network::router(state);

    let addr: SocketAddr= "127.0.0.1:9933".parse().unwrap();
    println!("🚀 UTILON node running at {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
