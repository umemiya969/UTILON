use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use crate::{
    consensus::try_finalize,
    state::SharedState,
    types::{Block, Job, Vote},
};

use std::collections::HashMap;

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/job", post(submit_job))
        .route("/vote", post(submit_vote))
        .route("/finalize", post(finalize_job))
        .route("/balances", get(get_balances))
        .route("/chain", get(get_chain))
        .with_state(state)
}

async fn submit_job(
    State(state): State<SharedState>,
    Json(job): Json<Job>,
) -> Json<&'static str> {
    state.jobs.lock().unwrap().insert(job.id.clone(), job);
    Json("job accepted")
}

async fn submit_vote(
    State(state): State<SharedState>,
    Json(vote): Json<Vote>,
) -> Json<&'static str> {
    state
        .votes
        .lock()
        .unwrap()
        .entry(vote.job_id.clone())
        .or_default()
        .push(vote);

    Json("vote accepted")
}

async fn finalize_job(
    State(state): State<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let job_id = payload["job_id"].as_str().unwrap();

    match try_finalize(job_id, &state) {
        Some(block) => Json(serde_json::json!({
            "status": "finalized",
            "block": block
        })),
        None => Json(serde_json::json!({
            "status": "pending"
        })),
    }
}

/* ✅ FIXED: TYPE KONKRET */
async fn get_balances(
    State(state): State<SharedState>,
) -> Json<HashMap<String, u64>> {
    Json(state.balances.lock().unwrap().clone())
}

/* ✅ FIXED: TYPE KONKRET */
async fn get_chain(
    State(state): State<SharedState>,
) -> Json<Vec<Block>> {
    Json(state.chain.lock().unwrap().clone())
}
