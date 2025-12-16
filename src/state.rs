use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::types::Job;

#[derive(Clone)]
pub struct AppState {
    pub jobs: Arc<Mutex<Vec<Job>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
