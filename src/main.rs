mod crypto;
mod network;
mod types;
mod state;

use axum::{Router, routing::post};
use state::AppState;
use crypto::generate_keypair;

#[tokio::main]
async fn main() {
    let keypair = generate_keypair();
    println!("🧑 Node pubkey: {}", hex::encode(keypair.verifying.to_bytes()));

    let state = AppState::new();

    let app = Router::new()
        .route("/message", post(network::receive_message))
        .with_state(state);

    println!("🚀 Node running on http://127.0.0.1:9933");
    axum::Server::bind(&"127.0.0.1:9933".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
