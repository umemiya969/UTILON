use crate::types::Block;

/// Return true jika incoming chain lebih baik dari local
pub fn is_better_chain(local: &Vec<Block>, incoming: &Vec<Block>) -> bool {
    // Rule 1: longest chain
    if incoming.len() > local.len() {
        return true;
    }

    if incoming.len() < local.len() {
        return false;
    }

    // Rule 2: tie-breaker (lowest hash)
    match (local.last(), incoming.last()) {
        (Some(l), Some(i)) => i.result_hash < l.result_hash,
        _ => false,
    }
}
