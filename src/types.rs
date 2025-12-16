use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    JobAnnouncement,
    JobResultSubmission,
    JobVerificationVote,
    BlockBroadcast,
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
}
