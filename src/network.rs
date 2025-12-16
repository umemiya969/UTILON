use axum::{Json, extract::State};
use crate::types::{NetworkEnvelope, MessageType, Job};
use crate::state::AppState;
use uuid::Uuid;

pub async fn receive_message(
    State(state): State<AppState>,
    Json(envelope): Json<NetworkEnvelope>,
) -> &'static str {
    let payload_bytes = envelope.payload.to_string().as_bytes().to_vec();

    // VALIDASI ENVELOPE (FASE 3)
    if envelope.sender_pubkey.is_empty() || envelope.signature.is_empty() {
        return "INVALID ENVELOPE";
    }

    match envelope.message_type {
        MessageType::JobAnnouncement => {
            if let Ok(job) = serde_json::from_value::<Job>(envelope.payload) {
                println!("📦 Job received: {}", job.job_id);
                state.jobs.lock().unwrap().push(job);
            }
        }
        _ => {
            println!("📨 Message received: {:?}", envelope.message_type);
        }
    }

    "OK"
}
