// examples/interactive_storytelling/environment_agent.rs

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::completion::{Chat, PromptError};

pub struct EnvironmentAgent<A: Chat> {
    pub inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> EnvironmentAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn describe_environment(
        &mut self,
        narrative_context: &str,
    ) -> Result<String, PromptError> {
        self.inner
            .transition_to(AgentState::Custom("DescribingEnvironment".into()));

        let prompt = format!(
            "Describe the environment based on the following narrative context:\n\n{}",
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
