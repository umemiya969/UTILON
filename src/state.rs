use std::collections::HashMap;
use crate::types::*;
use uuid::Uuid;

pub struct NodeState {
    pub jobs: HashMap<Uuid, Job>,
    pub votes: HashMap<Uuid, Vec<Vote>>,
    pub chain: Vec<Block>,
    pub balances: HashMap<String, u64>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
            votes: HashMap::new(),
            chain: Vec::new(),
            balances: HashMap::new(),
        }
    }
}
