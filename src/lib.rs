// src/lib.rs
pub mod error;
pub mod job;
pub mod runtime;
pub mod storage;
pub mod state_machine;

pub use error::Error;
pub use job::{Job, JobId, JobState};
pub use runtime::Runtime;
pub use storage::Storage;
pub use state_machine::{StateMachine, Transition};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_creation() {
        let runtime = Runtime::new();
        assert!(runtime.is_ok());
    }
}