// src/runtime.rs
use crate::{error::Error, job::Job, storage::Storage, state_machine::StateMachine};
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Runtime {
    storage: Arc<dyn Storage>,
    state_machines: Mutex<Vec<Box<dyn StateMachine>>>,
}

impl Runtime {
    pub fn new() -> Result<Self, Error> {
        // Default to in-memory storage for simplicity
        let storage = Arc::new(crate::storage::InMemoryStorage::new());
        Ok(Self {
            storage,
            state_machines: Mutex::new(Vec::new()),
        })
    }

    pub async fn register_state_machine(&self, sm: Box<dyn StateMachine>) {
        self.state_machines.lock().await.push(sm);
    }

    pub async fn submit_job(&self, job: Job) -> Result<JobId, Error> {
        let id = job.id.clone();
        self.storage.save_job(job).await?;
        Ok(id)
    }

    pub async fn get_job(&self, id: &JobId) -> Result<Job, Error> {
        self.storage.load_job(id).await
    }

    pub async fn update_job_state(&self, id: &JobId, state: crate::job::JobState) -> Result<(), Error> {
        let mut job = self.get_job(id).await?;
        job.set_state(state);
        self.storage.save_job(job).await?;
        Ok(())
    }

    pub async fn run_job(&self, id: &JobId) -> Result<(), Error> {
        let mut job = self.get_job(id).await?;
        if job.state != crate::job::JobState::Pending {
            return Err(Error::InvalidTransition(
                "Job must be pending to run".to_string(),
            ));
        }

        job.set_state(crate::job::JobState::Running);
        self.storage.save_job(job).await?;

        // Simulate async processing
        tokio::task::spawn(async move {
            // In a real implementation, this would execute the actual job logic
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });

        Ok(())
    }
}