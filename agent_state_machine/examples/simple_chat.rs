use agent_state_machine::{ChatAgentStateMachine, AgentState}; // Added AgentState import
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

    // Set up a response callback to handle outputs
    state_machine.set_response_callback(|response| {
        println!("🤖 Assistant: {}", response);
    });

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
        "What is the meaning of life?",
        "What is the airspeed velocity of an unladen swallow?",
        "What is the capital of Assyria?",
        "What is the airspeed velocity of a coconut-laden swallow?",
    ];

    // Enqueue all messages into the state machine
    for message in messages {
        println!("\n👤 User: {}", message);
        
        // Enqueue the message
        state_machine.process_message(message).await?;
    }

    // Wait until all messages have been processed
    while state_machine.current_state() != &AgentState::Ready {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
