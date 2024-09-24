mod flight_search_tool;

use crate::flight_search_tool::FlightSearchTool;
use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Build the agent with the FlightSearchTool
    let agent = openai_client
        .agent("gpt-4")
        .preamble("You are a travel assistant that can help users find flights between airports.")
        .tool(FlightSearchTool)
        .build();

    // query
    let response = agent
        .prompt("Find me flights from San Antonio (SAT) to London (LHR) on November 15th 2024.")
        .await?;

    // Deserialize the response to get the formatted string
    let formatted_response: String = serde_json::from_str(&response)?;

    println!("Agent response:\n{}", formatted_response);

    Ok(())
}