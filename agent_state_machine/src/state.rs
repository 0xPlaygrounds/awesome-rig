use std::fmt;

/// Represents the possible states of a chat agent
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    /// Ready to receive input
    Ready,
    /// Processing a user message
    Processing,
    /// Error state when something goes wrong
    Error(String),
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Ready => write!(f, "Ready"),
            AgentState::Processing => write!(f, "Processing"),
            AgentState::Error(msg) => write!(f, "Error: {}", msg),
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