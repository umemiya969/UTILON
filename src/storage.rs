use crate::types::Block;
use std::{fs, io::Write, path::Path};

const CHAIN_FILE: &str = "chain.json";

pub fn save_chain(chain: &Vec<Block>) {
    let json = match serde_json::to_string_pretty(chain) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("❌ Failed to serialize chain: {}", e);
            return;
        }
    };

    if let Err(e) = fs::File::create(CHAIN_FILE).and_then(|mut f| f.write_all(json.as_bytes())) {
        eprintln!("❌ Failed to save chain: {}", e);
    }
}

pub fn load_chain() -> Vec<Block> {
    if !Path::new(CHAIN_FILE).exists() {
        return Vec::new();
    }

    let data = match fs::read_to_string(CHAIN_FILE) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Failed to read chain file: {}", e);
            return Vec::new();
        }
    };

    match serde_json::from_str(&data) {
        Ok(chain) => chain,
        Err(e) => {
            eprintln!("❌ Failed to parse chain: {}", e);
            Vec::new()
        }
    }
}
