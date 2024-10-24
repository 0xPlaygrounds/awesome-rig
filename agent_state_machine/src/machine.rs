use crate::state::AgentState;
use rig::completion::{Chat, Message, PromptError};
use std::collections::VecDeque;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

/// A state machine for a chat agent that can process messages in a queue
pub struct ChatAgentStateMachine<A: Chat> {
    /// Current state of the agent
    current_state: AgentState,
    /// The underlying agent that handles the chat
    agent: A,
    /// Channel for broadcasting state changes
    state_tx: broadcast::Sender<AgentState>,
    /// Chat history
    history: Vec<Message>,
    /// Queue of messages to process
    queue: VecDeque<String>,
    /// Optional response callback to handle outputs
    response_callback: Option<Box<dyn Fn(String) + Send + Sync>>,
}

impl<A: Chat> ChatAgentStateMachine<A> {
    /// Create a new ChatAgentStateMachine with the given agent
    pub fn new(agent: A) -> Self {
        let (state_tx, _) = broadcast::channel(32);
        let machine = Self {
            current_state: AgentState::Ready,
            agent,
            state_tx,
            history: Vec::new(),
            queue: VecDeque::new(),
            response_callback: None,
        };

        info!("Agent initialized in state: {}", machine.current_state);

        machine
    }

    /// Set a response callback to handle outputs
    pub fn set_response_callback<F>(&mut self, callback: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.response_callback = Some(Box::new(callback));
    }

    /// Enqueue a user message for processing
    pub async fn process_message(&mut self, message: &str) -> Result<(), PromptError> {
        debug!("Enqueuing message: {}", message);
        self.queue.push_back(message.to_string());

        if self.current_state == AgentState::Ready {
            self.process_queue().await;
        }

        Ok(())
    }

    /// Process messages from the queue
    async fn process_queue(&mut self) {
        self.transition_to(AgentState::ProcessingQueue);

        while let Some(message) = self.queue.pop_front() {
            self.transition_to(AgentState::Processing);

            match self.process_single_message(&message).await {
                Ok(response) => {
                    // Handle the response (e.g., send it to the user)
                    if let Some(callback) = &self.response_callback {
                        callback(response);
                    } else {
                        println!("Response: {}", response);
                    }
                }
                Err(e) => {
                    error!("Error processing message: {}", e);
                    self.transition_to(AgentState::Error(e.to_string()));
                    // Decide whether to continue processing or break
                    // For this example, we'll break on error
                    break;
                }
            }
        }

        // After processing the queue, transition back to Ready
        self.transition_to(AgentState::Ready);
    }

    /// Process a single message
    async fn process_single_message(&mut self, message: &str) -> Result<String, PromptError> {
        debug!("Processing message: {}", message);

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
                debug!("Successfully processed message");
                Ok(response)
            }
            Err(e) => {
                error!("Error processing message: {}", e);
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
    use tokio::time::{sleep, Duration};

    struct MockAgent;

    impl Chat for MockAgent {
        fn chat<'a>(
            &'a self,
            prompt: &'a str,
            _history: Vec<Message>,
        ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'a>> {
            let response = format!("Echo: {}", prompt);
            Box::pin(async move {
                // Simulate some processing delay
                sleep(Duration::from_millis(50)).await;
                Ok(response)
            })
        }
    }

    #[tokio::test]
    async fn test_process_message_queue() {
        let mut machine = ChatAgentStateMachine::new(MockAgent);
        let mut responses = Vec::new();

        machine.set_response_callback(|response| {
            responses.push(response);
        });

        machine.process_message("Message 1").await.unwrap();
        machine.process_message("Message 2").await.unwrap();
        machine.process_message("Message 3").await.unwrap();

        // Wait until processing is complete
        while machine.current_state() != &AgentState::Ready {
            sleep(Duration::from_millis(10)).await;
        }

        assert_eq!(responses.len(), 3);
        assert_eq!(responses[0], "Echo: Message 1");
        assert_eq!(responses[1], "Echo: Message 2");
        assert_eq!(responses[2], "Echo: Message 3");
    }

    #[tokio::test]
    async fn test_clear_history() {
        let mut machine = ChatAgentStateMachine::new(MockAgent);
        machine.process_message("Test").await.unwrap();
        assert!(!machine.history().is_empty());
        machine.clear_history();
        assert!(machine.history().is_empty());
    }
}
