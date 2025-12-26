// src/storage.rs
use crate::{error::Error, job::Job};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_job(&self, job: Job) -> Result<(), Error>;
    async fn load_job(&self, id: &str) -> Result<Job, Error>;
    async fn list_jobs(&self) -> Result<Vec<Job>, Error>;
}

pub struct InMemoryStorage {
    jobs: tokio::sync::Mutex<HashMap<String, Job>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            jobs: tokio::sync::Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Storage for InMemoryStorage {
    async fn save_job(&self, job: Job) -> Result<(), Error> {
        self.jobs.lock().await.insert(job.id.clone(), job);
        Ok(())
    }

    async fn load_job(&self, id: &str) -> Result<Job, Error> {
        let jobs = self.jobs.lock().await;
        jobs.get(id)
            .cloned()
            .ok_or(Error::JobNotFound(id.to_string()))
    }

    async fn list_jobs(&self) -> Result<Vec<Job>, Error> {
        let jobs = self.jobs.lock().await;
        Ok(jobs.values().cloned().collect())
    }
}