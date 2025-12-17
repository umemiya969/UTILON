use reqwest::Client;
use crate::types::Block;

pub async fn push_chain(peer: &str, chain: &Vec<Block>) {
    let client = Client::new();
    let _ = client
        .post(format!("{peer}/p2p/chain"))
        .json(chain)
        .send()
        .await;
}
