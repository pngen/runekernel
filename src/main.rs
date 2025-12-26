// src/main.rs
use runekernel::{Runtime, job::Job};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    
    // Create and submit a job
    let mut job = Job::new();
    job.set_data("name".to_string(), serde_json::Value::String("test-job".to_string()));
    
    let job_id = runtime.submit_job(job).await?;
    println!("Submitted job with ID: {}", job_id);
    
    // Run the job
    runtime.run_job(&job_id).await?;
    println!("Job started");
    
    Ok(())
}