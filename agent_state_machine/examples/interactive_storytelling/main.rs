// examples/interactive_storytelling/main.rs

mod narrative_agent;
mod character_agent;
mod dialogue_agent;
mod environment_agent;

use narrative_agent::NarrativeAgent;
use character_agent::CharacterAgent;
use dialogue_agent::DialogueAgent;
use environment_agent::EnvironmentAgent;

use agent_state_machine::{ChatAgentStateMachine, AgentState};
use rig::providers::openai::{self, GPT_4};
use rig::completion::{Chat, PromptError};
use tokio::io::{self, AsyncBufReadExt};

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
        let stdin = io::BufReader::new(io::stdin());
        let mut lines = stdin.lines();

        let input = if let Ok(Some(line)) = lines.next_line().await {
            line
        } else {
            break;
        };

        if input.trim().is_empty() {
            break;
        }

        user_choice = Some(input.trim().to_string());
    }

    println!("\n=== The End ===");
    Ok(())
}
