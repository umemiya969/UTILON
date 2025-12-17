use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{
    state::NodeState,
    types::*,
    consensus::try_finalize,
    storage,
};

#[derive(serde::Deserialize)]
pub struct SubmitJobReq {
    pub payload: String,
    pub reward: u64,
}

#[derive(serde::Deserialize)]
pub struct VoteReq {
    pub job_id: Uuid,
    pub worker: String,
    pub result_hash: String,
}

pub fn router(state: Arc<Mutex<NodeState>>) -> Router {
    Router::new()
        .route("/job", post(submit_job))
        .route("/vote", post(vote))
        .route("/chain", get(get_chain))
        .route("/p2p/chain", post(sync_chain))
        .with_state(state)
}

async fn submit_job(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(req): Json<SubmitJobReq>,
) -> Json<Uuid> {
    let mut s = state.lock().unwrap();

    let job = Job {
        id: Uuid::new_v4(),
        payload: req.payload,
        reward: req.reward,
        finalized: false,
    };

    s.jobs.insert(job.id, job.clone());
    s.votes.insert(job.id, vec![]);

    Json(job.id)
}

async fn vote(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(req): Json<VoteReq>,
) -> Json<bool> {
    let mut s = state.lock().unwrap();

    s.votes.entry(req.job_id).or_default().push(Vote {
        job_id: req.job_id,
        worker: req.worker,
        result_hash: req.result_hash,
    });

    let finalized = try_finalize(&mut s, req.job_id).is_some();
    storage::save_chain(&s.chain).ok();

    Json(finalized)
}

async fn get_chain(
    State(state): State<Arc<Mutex<NodeState>>>,
) -> Json<Vec<Block>> {
    let s = state.lock().unwrap();
    Json(s.chain.clone())
}

async fn sync_chain(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(remote): Json<Vec<Block>>,
) {
    let mut s = state.lock().unwrap();
    if remote.len() > s.chain.len() {
        s.chain = remote;
        storage::save_chain(&s.chain).ok();
    }
}
