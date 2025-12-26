# RuneKernel

A deterministic job runtime with declarative state-machine definitions, futures-aware concurrency, and pluggable storage.

## Overview

RuneKernel is a low-level, highly technical runtime for managing concurrent jobs with deterministic execution semantics. It provides:

- **Declarative State Machines**: Define job lifecycle transitions explicitly
- **Pluggable Storage**: Support for SQLite, Postgres, RocksDB, or custom backends
- **Tokio-backed Scheduler**: Efficient async job execution with cancellation support
- **Axum Admin API**: RESTful interface for job management
- **Execution Graph Visualizer**: WebAssembly-based UI for monitoring workflows

## Architecture
```markdown
┌─────────────┐    ┌──────────────┐    ┌──────────────┐
│   Runtime   │────│State Machine │────│   Storage    │
└─────────────┘    └──────────────┘    └──────────────┘
       │                   │                │
       ▼                   ▼                ▼
┌─────────────┐    ┌──────────────┐    ┌──────────────┐
│   Job API   │    │ Transition   │    │  Storage     │
│             │    │ Validation   │    │ Adapters     │
└─────────────┘    └──────────────┘    └──────────────┘
```
## Core Components

### Job Management
- `Job`: Represents a unit of work with state tracking
- `JobId`: Unique identifier for jobs
- `JobState`: Lifecycle states (Pending, Running, Completed, Failed, Cancelled)

### State Machines
- `StateMachine`: Defines valid job transitions
- `Transition`: Describes allowed state changes with optional conditions

### Storage Layer
- `Storage`: Trait for persistent job storage
- `InMemoryStorage`: Default in-memory implementation
- Pluggable backends for SQLite, Postgres, RocksDB

### Runtime
- `Runtime`: Main execution context managing jobs and state transitions
- Async-safe operations with Tokio concurrency model

## Quick Start

```rust
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
```

## Features

### Deterministic Execution
- Predictable job lifecycle management
- Explicit state transition validation
- Idempotent operations where possible

### Concurrency Model
- Tokio-based async execution
- Cancellation support for running jobs
- Thread-safe storage operations

### Storage Flexibility
```rust
// Switch to SQLite backend
use runekernel::storage::SqliteStorage;
let storage = SqliteStorage::new("jobs.db");
```

### State Machine Definition
```rust
use runekernel::state_machine::{StateMachine, Transition};

let sm = SimpleStateMachine::new("example");
let transitions = sm.transitions();
```

## API Endpoints (Axum Admin)

- `POST /jobs` - Submit new job
- `GET /jobs/{id}` - Get job details
- `PUT /jobs/{id}/state` - Update job state
- `POST /jobs/{id}/run` - Execute job
- `GET /jobs` - List all jobs

## Testing

Comprehensive property-based testing with proptest for safety invariants:

```bash
cargo test
```

## Performance Considerations

- Minimal memory overhead
- Efficient async scheduling
- Lock-free operations where possible
- Batch processing capabilities

## Security

- Input validation for job data
- State transition authorization
- Thread-safe concurrent access
- Error isolation between components

## Extensibility

### Custom Storage Backend
```rust
struct MyStorage { /* implementation */ }

#[async_trait]
impl Storage for MyStorage {
    // Implement required methods
}
```

### Custom State Machine
```rust
struct MyStateMachine { /* implementation */ }

impl StateMachine for MyStateMachine {
    // Implement state transition logic
}
```

## Requirements

- Rust 1.56+
- Tokio runtime
- Async support (async-std or tokio)

## Contributing

1. Fork the repository
2. Create feature branch
3. Add tests for new functionality
4. Submit pull request

## License

MIT License

## Author

Paul Ngen
