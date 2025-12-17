use crate::types::*;
use std::collections::HashSet;

pub fn validate_chain(chain: &Vec<Block>) -> bool {
    let mut seen_jobs = HashSet::new();

    for (i, block) in chain.iter().enumerate() {
        // 1. index harus urut
        if block.index != i as u64 {
            eprintln!("❌ Invalid index at block {}", i);
            return false;
        }

        // 2. job_id tidak boleh dobel
        if !seen_jobs.insert(block.job_id) {
            eprintln!("❌ Duplicate job_id {}", block.job_id);
            return false;
        }

        // 3. reward harus masuk akal
        if block.reward == 0 {
            eprintln!("❌ Zero reward at block {}", block.index);
            return false;
        }

        // 4. hash tidak boleh kosong
        if block.result_hash.is_empty() {
            eprintln!("❌ Empty result_hash at block {}", block.index);
            return false;
        }
    }

    true
}
