use std::collections::HashMap;
use uuid::Uuid;
use crate::types::*;

#[derive(Default)]
pub struct NodeState {
    pub jobs: HashMap<Uuid, Job>,
    pub votes: HashMap<Uuid, Vec<Vote>>,
    pub chain: Vec<Block>,
    pub balances: HashMap<String, u64>,
}

impl NodeState {
    pub fn new() -> Self {
        Self::default()
    }
}
