use axum::{Json, extract::State};
use crate::types::{NetworkEnvelope, MessageType, Job, Vote};
use crate::state::AppState;
use crate::consensus::try_finalize;

pub async fn receive_message(
    State(state): State<AppState>,
    Json(envelope): Json<NetworkEnvelope>,
) -> &'static str {

    match envelope.message_type {
        MessageType::JobAnnouncement => {
            if let Ok(job) = serde_json::from_value::<Job>(envelope.payload) {
                println!("📦 Job announced: {}", job.job_id);
                state.jobs.lock().unwrap().push(job);
            }
        }

        MessageType::JobVerificationVote => {
    if let Ok(vote) = serde_json::from_value::<Vote>(envelope.payload) {
        let job_id = vote.job_id; // ambil dulu

        println!("🗳️ Vote received for job {}", job_id);

        state.votes.lock().unwrap().push(vote);

        let mut jobs = state.jobs.lock().unwrap();
        let votes = state.votes.lock().unwrap();

        if let Some(job) = jobs.iter_mut().find(|j| j.job_id == job_id) {
            try_finalize(job, &votes);
        }
    }
}


        _ => {}
    }

    "OK"
}
