use std::{fs, io::Result};
use crate::types::Block;

const CHAIN_FILE: &str = "chain.json";

pub fn load_chain() -> Vec<Block> {
    if let Ok(data) = fs::read_to_string(CHAIN_FILE) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_chain(chain: &Vec<Block>) -> Result<()> {
    let data = serde_json::to_string_pretty(chain).unwrap();
    fs::write(CHAIN_FILE, data)
}
