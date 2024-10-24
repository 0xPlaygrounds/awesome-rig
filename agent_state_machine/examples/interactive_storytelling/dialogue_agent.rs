// examples/interactive_storytelling/dialogue_agent.rs

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::completion::{Chat, PromptError};

pub struct DialogueAgent<A: Chat> {
    pub inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> DialogueAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn generate_dialogue(
        &mut self,
        character_context: &str,
    ) -> Result<String, PromptError> {
        self.inner
            .transition_to(AgentState::Custom("GeneratingDialogue".into()));

        let prompt = format!(
            "Generate a dialogue between characters based on the following context:\n\n{}",
            character_context
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
