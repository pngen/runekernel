// src/state_machine.rs
use crate::{error::Error, job::JobState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: JobState,
    pub to: JobState,
    pub condition: Option<String>,
}

pub trait StateMachine: Send + Sync {
    fn name(&self) -> &str;
    fn transitions(&self) -> Vec<Transition>;
    fn can_transition(&self, from: &JobState, to: &JobState) -> bool;
    fn validate_transition(&self, from: &JobState, to: &JobState) -> Result<(), Error>;
}

pub struct SimpleStateMachine {
    name: String,
    transitions: Vec<Transition>,
}

impl SimpleStateMachine {
    pub fn new(name: &str) -> Self {
        let transitions = vec![
            Transition {
                from: JobState::Pending,
                to: JobState::Running,
                condition: None,
            },
            Transition {
                from: JobState::Running,
                to: JobState::Completed,
                condition: None,
            },
            Transition {
                from: JobState::Running,
                to: JobState::Failed,
                condition: None,
            },
        ];

        Self {
            name: name.to_string(),
            transitions,
        }
    }
}

impl StateMachine for SimpleStateMachine {
    fn name(&self) -> &str {
        &self.name
    }

    fn transitions(&self) -> Vec<Transition> {
        self.transitions.clone()
    }

    fn can_transition(&self, from: &JobState, to: &JobState) -> bool {
        self.transitions
            .iter()
            .any(|t| t.from == *from && t.to == *to)
    }

    fn validate_transition(&self, from: &JobState, to: &JobState) -> Result<(), Error> {
        if !self.can_transition(from, to) {
            return Err(Error::InvalidTransition(
                format!("Cannot transition from {:?} to {:?}", from, to),
            ));
        }
        Ok(())
    }
}