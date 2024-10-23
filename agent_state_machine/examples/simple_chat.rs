use agent_state_machine::ChatAgentStateMachine;
use rig::providers::openai::{self, GPT_4};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Chat Agent State Machine Demo ===\n");
    
    // Create OpenAI client
    let client = openai::Client::from_env();

    // Create a basic chat agent
    let agent = client
        .agent(GPT_4)
        .preamble("\
            You are a helpful and friendly AI assistant. \
            Keep your responses concise but engaging.\
        ")
        .build();

    // Create state machine
    let mut state_machine = ChatAgentStateMachine::new(agent);

    // Subscribe to state changes
    let mut state_rx = state_machine.subscribe_to_state_changes();

    // Spawn task to monitor state changes
    tokio::spawn(async move {
        while let Ok(state) = state_rx.recv().await {
            println!("📍 State: {}", state);
        }
    });

    // Process a few messages
    let messages = vec![
        "Hello! How are you?",
        "What's your favorite color?",
        "what is the meaning of life?",
        "what is the airspeed velocity of an unladen swallow?",
        "what is the capital of Assyria?",
        "what is the airspeed velocity of a coconut-laden swallow?",
    ];

    for message in messages {
        println!("\n👤 User: {}", message);
        
        match state_machine.process_message(message).await {
            Ok(response) => {
                println!("🤖 Assistant: {}", response);
                // Small delay to make the conversation feel more natural
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}