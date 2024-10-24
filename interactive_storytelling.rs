// examples/interactive_storytelling.rs

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::providers::openai::{self, GPT_4};
use rig::completion::Chat;
use rig::completion::PromptError;
// use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt};

struct NarrativeAgent<A: Chat> {
    inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> NarrativeAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn generate_plot(&mut self, user_choice: Option<String>) -> Result<String, PromptError> {
        self.inner.transition_to(AgentState::Custom("GeneratingPlot".into()));

        let prompt = match user_choice {
            Some(choice) => format!("Based on the user's choice '{}', continue the story.", choice),
            None => "Start a new interactive story in the fantasy genre.".to_string(),
        };

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner.transition_to(AgentState::Custom("WaitingForChoice".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}

struct CharacterAgent<A: Chat> {
    inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> CharacterAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn update_characters(&mut self, narrative_context: &str) -> Result<String, PromptError> {
        self.inner.transition_to(AgentState::Custom("UpdatingCharacters".into()));

        let prompt = format!(
            "Based on the following narrative context, update the characters' states and actions:\n\n{}",
            narrative_context
        );

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner.transition_to(AgentState::Custom("Completed".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}

struct DialogueAgent<A: Chat> {
    inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> DialogueAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn generate_dialogue(&mut self, character_context: &str) -> Result<String, PromptError> {
        self.inner.transition_to(AgentState::Custom("GeneratingDialogue".into()));

        let prompt = format!(
            "Generate a dialogue between characters based on the following context:\n\n{}",
            character_context
        );

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner.transition_to(AgentState::Custom("Completed".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}

struct EnvironmentAgent<A: Chat> {
    inner: ChatAgentStateMachine<A>,
}

impl<A: Chat> EnvironmentAgent<A> {
    pub fn new(agent: A) -> Self {
        Self {
            inner: ChatAgentStateMachine::new(agent),
        }
    }

    pub async fn describe_environment(&mut self, narrative_context: &str) -> Result<String, PromptError> {
        self.inner.transition_to(AgentState::Custom("DescribingEnvironment".into()));

        let prompt = format!(
            "Describe the environment based on the following narrative context:\n\n{}",
            narrative_context
        );

        let response = self.inner.process_single_message(&prompt).await?;

        self.inner.transition_to(AgentState::Custom("Completed".into()));
        Ok(response)
    }

    pub fn current_state(&self) -> &AgentState {
        self.inner.current_state()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Interactive Storytelling Demo ===\n");

    // Create OpenAI client
    let client = openai::Client::from_env();

    // Initialize agents
    let narrative_agent = client
        .agent(GPT_4)
        .preamble("You are a Narrative Agent that creates engaging stories.")
        .build();
    let mut narrative_state_machine = NarrativeAgent::new(narrative_agent);

    let character_agent = client
        .agent(GPT_4)
        .preamble("You are a Character Agent that develops characters in a story.")
        .build();
    let mut character_state_machine = CharacterAgent::new(character_agent);

    let dialogue_agent = client
        .agent(GPT_4)
        .preamble("You are a Dialogue Agent that generates dialogues between characters.")
        .build();
    let mut dialogue_state_machine = DialogueAgent::new(dialogue_agent);

    let environment_agent = client
        .agent(GPT_4)
        .preamble("You are an Environment Agent that describes settings vividly.")
        .build();
    let mut environment_state_machine = EnvironmentAgent::new(environment_agent);

    // Start the story
    let mut user_choice: Option<String> = None;
    loop {
        // Generate plot
        let narrative_output = narrative_state_machine.generate_plot(user_choice.clone()).await?;
        println!("\n📖 Narrative:\n{}\n", narrative_output);

        // Update characters
        let character_output = character_state_machine.update_characters(&narrative_output).await?;
        println!("👥 Characters:\n{}\n", character_output);

        // Describe environment
        let environment_output = environment_state_machine.describe_environment(&narrative_output).await?;
        println!("🌄 Environment:\n{}\n", environment_output);

        // Generate dialogue
        let dialogue_output = dialogue_state_machine.generate_dialogue(&character_output).await?;
        println!("💬 Dialogue:\n{}\n", dialogue_output);

        // Present the combined story segment to the user
        println!("=== Story Segment ===");
        println!("{}\n{}\n{}\n", environment_output, narrative_output, dialogue_output);

        // Ask for user input
        println!("What do you want to do next?");
        let mut input = String::new();
        let stdin = io::BufReader::new(io::stdin());
        let mut lines = stdin.lines();
        if let Ok(Some(line)) = lines.next_line().await {
            input = line;
        } else {
            break;
        }        

        if input.trim().is_empty() {
            break;
        }

        user_choice = Some(input.trim().to_string());
    }

    println!("\n=== The End ===");
    Ok(())
}
