use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub payload: String,
    pub reward: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vote {
    pub job_id: String,
    pub worker: String,
    pub result_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub job_id: String,
    pub result_hash: String,
    pub rewards: Vec<(String, u64)>,
}
