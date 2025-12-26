// tests/integration_tests.rs
use runekernel::{Runtime, job::Job};

#[tokio::test]
async fn test_job_submission_and_execution() {
    let runtime = Runtime::new().unwrap();
    
    // Submit a job
    let mut job = Job::new();
    job.set_data("name".to_string(), serde_json::Value::String("integration-test".to_string()));
    
    let job_id = runtime.submit_job(job).await.unwrap();
    
    // Verify job exists
    let loaded_job = runtime.get_job(&job_id).await.unwrap();
    assert_eq!(loaded_job.id, job_id);
    assert_eq!(loaded_job.state, runekernel::job::JobState::Pending);
    
    // Run the job
    runtime.run_job(&job_id).await.unwrap();
    
    // Verify state change
    let updated_job = runtime.get_job(&job_id).await.unwrap();
    assert_eq!(updated_job.state, runekernel::job::JobState::Running);
}