// src/error.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    StorageError(String),
    JobNotFound(JobId),
    InvalidTransition(String),
    SerializationError(String),
    InternalError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::StorageError(msg) => write!(f, "Storage error: {}", msg),
            Error::JobNotFound(id) => write!(f, "Job not found: {}", id),
            Error::InvalidTransition(msg) => write!(f, "Invalid transition: {}", msg),
            Error::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerializationError(error.to_string())
    }
}