use crate::{state::SharedState, types::*};

pub fn try_finalize(job_id: &str, state: &SharedState) -> Option<Block> {
    let mut finalized = state.finalized.lock().unwrap();
    if finalized.contains(job_id) {
        return None;
    }

    let votes = state.votes.lock().unwrap();
    let job_votes = votes.get(job_id)?;

    if job_votes.len() < 2 {
        return None;
    }

    let hash = &job_votes[0].result_hash;
    if !job_votes.iter().all(|v| &v.result_hash == hash) {
        return None;
    }

    let reward_each = {
        let jobs = state.jobs.lock().unwrap();
        let job = jobs.get(job_id)?;
        job.reward / job_votes.len() as u64
    };

    let mut balances = state.balances.lock().unwrap();
    let rewards: Vec<(String, u64)> = job_votes
        .iter()
        .map(|v| {
            let entry = balances.entry(v.worker.clone()).or_insert(0);
            *entry += reward_each;
            (v.worker.clone(), reward_each)
        })
        .collect();

    let mut chain = state.chain.lock().unwrap();
    let block = Block {
        index: chain.len() as u64,
        job_id: job_id.to_string(),
        result_hash: hash.clone(),
        rewards,
    };

    chain.push(block.clone());
    finalized.insert(job_id.to_string());

    Some(block)
}
