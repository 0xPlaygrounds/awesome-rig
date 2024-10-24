// examples/interactive_storytelling/character_agent.rs

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::completion::{Chat, PromptError};

pub struct CharacterAgent<A: Chat> {
    pub inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> CharacterAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn update_characters(
        &mut self,
        narrative_context: &str,
    ) -> Result<String, PromptError> {
        self.inner
            .transition_to(AgentState::Custom("UpdatingCharacters".into()));

        let prompt = format!(
            "Based on the following narrative context, update the characters' states and actions:\n\n{}",
            narrative_context
        );

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner
            .transition_to(AgentState::Custom("Completed".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}
