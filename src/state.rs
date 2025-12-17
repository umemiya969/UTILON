use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::types::*;

pub struct NodeState {
    pub jobs: Mutex<HashMap<String, Job>>,
    pub votes: Mutex<HashMap<String, Vec<Vote>>>,
    pub finalized: Mutex<HashSet<String>>,
    pub chain: Mutex<Vec<Block>>,
    pub balances: Mutex<HashMap<String, u64>>,
}

pub type SharedState = Arc<NodeState>;
