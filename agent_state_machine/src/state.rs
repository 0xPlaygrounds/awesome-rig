// src/state.rs

use std::fmt;

/// Represents the possible states of a chat agent
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    /// Ready to receive input
    Ready,
    /// Processing a user message
    Processing,
    /// Processing messages from the queue
    ProcessingQueue,
    /// Error state when something goes wrong
    Error(String),
    /// Custom state for specific agent actions
    Custom(String),
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Ready => write!(f, "Ready"),
            AgentState::Processing => write!(f, "Processing"),
            AgentState::ProcessingQueue => write!(f, "Processing Queue"),
            AgentState::Error(msg) => write!(f, "Error: {}", msg),
            AgentState::Custom(state) => write!(f, "{}", state),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_display() {
        assert_eq!(AgentState::Ready.to_string(), "Ready");
        assert_eq!(AgentState::Processing.to_string(), "Processing");
        assert_eq!(
            AgentState::Error("test error".into()).to_string(),
            "Error: test error"
        );
    }

    #[test]
    fn test_state_clone_and_eq() {
        let state = AgentState::Ready;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }
}