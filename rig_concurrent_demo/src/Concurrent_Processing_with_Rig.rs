// Concurrent Processing with Rig

use rig::providers::openai;  // Import OpenAI provider from Rig
use rig::completion::Prompt;  // Import Prompt trait for LLM interactions
use tokio::task;  // Import Tokio's task spawning functionality
use std::time::Instant;  // For measuring execution time
use std::sync::Arc;  // For thread-safe sharing of the model

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OpenAI client using environment variables
    let openai_client = openai::Client::from_env();
    
    // Create a GPT-3.5-turbo model instance and wrap it in an Arc for thread-safe sharing
    let model = Arc::new(openai_client.model("gpt-3.5-turbo").build());

    // Start timing the execution
    let start = Instant::now();
    
    // Vector to store task handles
    let mut handles = vec![];

    // Spawn 10 concurrent tasks
    for i in 0..10 {
        // Clone the Arc<Model> for each task
        let model_clone = Arc::clone(&model);
        
        // Spawn an asynchronous task
        let handle = task::spawn(async move {
            // Create a unique prompt for each task
            let prompt = format!("Generate a random fact about the number {}", i);
            // Use the cloned model to send a prompt to the LLM
            model_clone.prompt(&prompt).await
        });
        
        // Store the task handle
        handles.push(handle);
    }

    // Collect and process results
    for handle in handles {
        // Await the completion of each task
        // The first '?' unwraps the JoinError (if the task panicked)
        // The second '?' unwraps the Result from the prompt method
        let result = handle.await??;
        println!("Result: {}", result);
    }

    // Print the total execution time
    println!("Time elapsed: {:?}", start.elapsed());
    
    Ok(())
}