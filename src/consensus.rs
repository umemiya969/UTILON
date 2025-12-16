use crate::types::{Job, Vote};
use std::collections::HashMap;

pub fn try_finalize(job: &mut Job, votes: &[Vote]) {
    if job.finalized {
        return;
    }

    let mut counter: HashMap<String, usize> = HashMap::new();

    for v in votes {
        *counter.entry(v.result_hash.clone()).or_insert(0) += 1;
    }

    for (result, count) in counter {
        if count >= job.required_votes {
            job.finalized = true;
            job.final_result = Some(result);
            println!("✅ Job {} FINALIZED", job.job_id);
            return;
        }
    }
}
