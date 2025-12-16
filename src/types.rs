use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub payload: String,
    pub required_votes: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vote {
    pub job_id: Uuid,
    pub worker: String,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub job_id: Uuid,
    pub result: String,
}
