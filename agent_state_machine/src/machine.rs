use crate::state::AgentState;
use rig::completion::{Chat, Message, PromptError};
use tokio::sync::broadcast;
use tracing::{debug, error, info};

/// A simple state machine for a chat agent
pub struct ChatAgentStateMachine<A: Chat> {
    /// Current state of the agent
    current_state: AgentState,
    /// The underlying agent that handles the chat
    agent: A,
    /// Channel for broadcasting state changes
    state_tx: broadcast::Sender<AgentState>,
    /// Chat history
    history: Vec<Message>,
}

impl<A: Chat> ChatAgentStateMachine<A> {
    /// Create a new ChatAgentStateMachine with the given agent
    pub fn new(agent: A) -> Self {
        let (state_tx, _) = broadcast::channel(32);
        let machine = Self {  // Removed 'mut' as it's not needed
            current_state: AgentState::Ready,
            agent,
            state_tx,
            history: Vec::new(),
        };
        
        info!("Agent initialized in state: {}", machine.current_state);
        
        machine
    }

    /// Process a user message through the state machine
    pub async fn process_message(&mut self, message: &str) -> Result<String, PromptError> {
        debug!("Processing message: {}", message);
        self.transition_to(AgentState::Processing);

        self.history.push(Message {
            role: "user".into(),
            content: message.into(),
        });

        match self.agent.chat(message, self.history.clone()).await {
            Ok(response) => {
                self.history.push(Message {
                    role: "assistant".into(),
                    content: response.clone(),
                });

                self.transition_to(AgentState::Ready);
                debug!("Successfully processed message");
                Ok(response)
            }
            Err(e) => {
                error!("Error processing message: {}", e);
                self.transition_to(AgentState::Error(e.to_string()));
                Err(e)
            }
        }
    }

    /// Get the current state
    pub fn current_state(&self) -> &AgentState {
        &self.current_state
    }

    /// Get the chat history
    pub fn history(&self) -> &[Message] {
        &self.history
    }

    /// Subscribe to state changes
    pub fn subscribe_to_state_changes(&self) -> broadcast::Receiver<AgentState> {
        self.state_tx.subscribe()
    }

    /// Clear the chat history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    fn transition_to(&mut self, new_state: AgentState) {
        debug!("State transition: {} -> {}", self.current_state, new_state);
        self.current_state = new_state.clone();
        let _ = self.state_tx.send(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;

    struct MockAgent;

    impl Chat for MockAgent {
        fn chat<'a>(
            &'a self,
            _prompt: &'a str,
            _history: Vec<Message>,
        ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'a>> {
            Box::pin(async { Ok("Mock response".to_string()) })
        }
    }

    #[tokio::test]
    async fn test_process_message() {
        let mut machine = ChatAgentStateMachine::new(MockAgent);
        let response = machine.process_message("Test").await.unwrap();
        assert_eq!(response, "Mock response");
        assert_eq!(*machine.current_state(), AgentState::Ready);
        assert_eq!(machine.history().len(), 2);
    }

    #[tokio::test]
    async fn test_clear_history() {
        let mut machine = ChatAgentStateMachine::new(MockAgent);
        machine.process_message("Test").await.unwrap();
        assert_eq!(machine.history().len(), 2);
        machine.clear_history();
        assert_eq!(machine.history().len(), 0);
    }
}