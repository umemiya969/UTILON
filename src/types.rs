use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    JobAnnouncement,
    JobResultSubmission,
    JobVerificationVote,
    BlockBroadcast, // belum dipakai
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkEnvelope {
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub sender_pubkey: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub job_id: Uuid,
    pub payload: String,
    pub required_votes: usize,
    pub finalized: bool,
    pub final_result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    pub job_id: Uuid,
    pub worker_pubkey: String,
    pub result_hash: String,
    pub signature: String,
}
