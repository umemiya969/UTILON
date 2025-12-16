use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::types::{Job, Vote};

#[derive(Clone)]
pub struct AppState {
    pub jobs: Arc<Mutex<Vec<Job>>>,
    pub votes: Arc<Mutex<Vec<Vote>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
            votes: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
