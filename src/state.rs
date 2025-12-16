use crate::types::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use uuid::Uuid;

pub struct NodeState {
    pub jobs: Mutex<HashMap<Uuid, Job>>,
    pub votes: Mutex<HashMap<Uuid, Vec<Vote>>>,
    pub chain: Mutex<Vec<Block>>,
    pub finalized: Mutex<HashSet<Uuid>>, // 🔒 FASE 5 LOCK
}
