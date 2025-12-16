use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::{
    consensus::try_finalize,
    state::NodeState,
    types::{Block, Job, Vote},
};

pub fn router(state: Arc<NodeState>) -> Router {
    Router::new()
        .route("/job/submit", post(submit_job))
        .route("/job/next", get(next_job))
        .route("/job/vote", post(submit_vote))
        .route("/chain", get(get_chain))
        .with_state(state)
}

async fn submit_job(
    State(state): State<Arc<NodeState>>,
    Json(job): Json<Job>,
) -> Json<&'static str> {
    state.jobs.lock().unwrap().insert(job.id, job);
    Json("ok")
}

async fn next_job(
    State(state): State<Arc<NodeState>>,
) -> Json<Option<Job>> {
    let jobs = state.jobs.lock().unwrap();
    let finalized = state.finalized.lock().unwrap();

    let job = jobs
        .values()
        .find(|j| !finalized.contains(&j.id))
        .cloned();

    Json(job)
}

async fn submit_vote(
    State(state): State<Arc<NodeState>>,
    Json(vote): Json<Vote>,
) -> Json<&'static str> {
    if state.finalized.lock().unwrap().contains(&vote.job_id) {
        return Json("rejected: finalized");
    }

    state
        .votes
        .lock()
        .unwrap()
        .entry(vote.job_id)
        .or_default()
        .push(vote.clone());

    if let Some(block) = try_finalize(&state, vote.job_id) {
        println!("⛓ FINALIZED BLOCK #{}", block.index);
    }

    Json("ok")
}

async fn get_chain(
    State(state): State<Arc<NodeState>>,
) -> Json<Vec<Block>> {
    let chain = state.chain.lock().unwrap();
    Json(chain.clone())
}
