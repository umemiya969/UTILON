use crate::{state::NodeState, types::*};

pub fn try_finalize(state: &mut NodeState, job_id: uuid::Uuid) -> Option<Block> {
    let job = state.jobs.get_mut(&job_id)?;

    if job.finalized {
        return None;
    }

    let votes = state.votes.get(&job_id)?;
    if votes.len() < 2 {
        return None;
    }

    let hash = &votes[0].result_hash;
    if !votes.iter().all(|v| &v.result_hash == hash) {
        return None;
    }

    let block = Block {
        index: state.chain.len() as u64,
        job_id,
        result_hash: hash.clone(),
        reward: job.reward,
    };

    job.finalized = true;
    state.chain.push(block.clone());

    use std::collections::HashSet;

let mut rewarded = HashSet::new();

for v in votes {
    if rewarded.insert(&v.worker) {
        *state.balances
            .entry(v.worker.clone())
            .or_insert(0) += job.reward;
    }
}


    Some(block)
}
