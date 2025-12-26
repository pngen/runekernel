// src/job.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type JobId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub id: JobId,
    pub state: JobState,
    pub data: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl Default for Job {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            state: JobState::Pending,
            data: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Job {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(id: JobId) -> Self {
        let mut job = Self::default();
        job.id = id;
        job
    }

    pub fn set_state(&mut self, state: JobState) {
        self.state = state;
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
        self.updated_at = chrono::Utc::now();
    }
}