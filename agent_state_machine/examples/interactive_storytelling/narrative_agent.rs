// examples/interactive_storytelling/narrative_agent.rs

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::completion::{Chat, PromptError};

pub struct NarrativeAgent<A: Chat> {
    pub inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> NarrativeAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn generate_plot(
        &mut self,
        user_choice: Option<String>,
    ) -> Result<String, PromptError> {
        self.inner
            .transition_to(AgentState::Custom("GeneratingPlot".into()));

        let prompt = match user_choice {
            Some(choice) => format!("Based on the user's choice '{}', continue the story.", choice),
            None => "Start a new interactive story in the fantasy genre.".to_string(),
        };

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner
            .transition_to(AgentState::Custom("WaitingForChoice".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}
