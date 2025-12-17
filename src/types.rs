use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub payload: String,
    pub reward: u64,
    pub finalized: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vote {
    pub job_id: Uuid,
    pub worker: String,
    pub result_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub job_id: Uuid,
    pub result_hash: String,
    pub reward: u64,
}
