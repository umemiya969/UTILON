use crate::{state::NodeState, types::*};
use uuid::Uuid;

pub fn try_finalize(
    state: &NodeState,
    job_id: Uuid,
) -> Option<Block> {
    let votes = state.votes.lock().unwrap();
    let jobs = state.jobs.lock().unwrap();
    let finalized = state.finalized.lock().unwrap();

    if finalized.contains(&job_id) {
        return None;
    }

    let job = jobs.get(&job_id)?;
    let vote_count = votes.get(&job_id)?.len();

    if vote_count < job.required_votes as usize {
        return None;
    }

    drop(votes);
    drop(jobs);
    drop(finalized);

    let mut chain = state.chain.lock().unwrap();
    let index = chain.len() as u64;

    let result = state
        .votes
        .lock()
        .unwrap()
        .get(&job_id)
        .unwrap()[0]
        .result
        .clone();

    let block = Block {
        index,
        job_id,
        result,
    };

    chain.push(block.clone());

    // 🔒 FINALIZATION LOCK
    state.finalized.lock().unwrap().insert(job_id);
    state.jobs.lock().unwrap().remove(&job_id);
    state.votes.lock().unwrap().remove(&job_id);

    Some(block)
}
