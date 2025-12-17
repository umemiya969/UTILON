use axum::{Router, routing::{post, get}, Json, extract::State};
use std::sync::{Arc, Mutex};
use crate::{state::NodeState, types::*, consensus::try_finalize};
use uuid::Uuid;
use crate::validate::validate_chain;

pub fn router(state: Arc<Mutex<NodeState>>) -> Router {
    Router::new()
        .route("/job", post(submit_job))
        .route("/vote", post(submit_vote))
        .route("/finalize", post(finalize))
        .route("/chain", get(get_chain))
        .route("/balance", get(get_balance))
        .with_state(state)
}

async fn submit_job(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(mut job): Json<Job>,
) -> Json<Job> {
    job.id = Uuid::new_v4();
    state.lock().unwrap().jobs.insert(job.id, job.clone());
    Json(job)
}

async fn submit_vote(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(vote): Json<Vote>,
) -> Json<&'static str> {
    let mut s = state.lock().unwrap();
    let votes = s.votes.entry(vote.job_id).or_default();

    if votes.iter().any(|v| v.worker == vote.worker) {
        return Json("duplicate_vote");
    }

    votes.push(vote);
    Json("ok")
}


async fn finalize(
    State(state): State<Arc<Mutex<NodeState>>>,
    Json(id): Json<Uuid>,
) -> Json<&'static str> {
    let mut s = state.lock().unwrap();
    match try_finalize(&mut s, id) {
        Some(_) => Json("finalized"),
        None => Json("not_ready"),
    }
}

async fn get_chain(
    State(state): State<Arc<Mutex<NodeState>>>,
) -> Json<Vec<Block>> {
    Json(state.lock().unwrap().chain.clone())
}

async fn get_balance(
    State(state): State<Arc<Mutex<NodeState>>>,
) -> Json<std::collections::HashMap<String, u64>> {
    Json(state.lock().unwrap().balances.clone())
}


pub fn receive_chain(state: &mut NodeState, incoming: Vec<Block>) {
    if !validate_chain(&incoming) {
        println!("⛔ Rejected invalid chain from peer");
        return;
    }

    if incoming.len() > state.chain.len() {
        println!("🔄 Chain updated from peer");
        state.chain = incoming;
    }
}
