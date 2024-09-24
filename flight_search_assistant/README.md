# Flight Search AI Assistant

Welcome to the **Flight Search AI Assistant** project! This application is an AI-powered assistant built with Rust using the [Rig](https://github.com/riggoio/rig) library. It allows users to find the cheapest flights between two airports through natural language queries.

## Table of Contents

- [Flight Search AI Assistant](#flight-search-ai-assistant)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Getting Started](#getting-started)
    - [Clone the Repository](#clone-the-repository)
    - [Set Up Environment Variables](#set-up-environment-variables)
  - [Build and Run](#build-and-run)
    - [Install Dependencies](#install-dependencies)
    - [Build the Project](#build-the-project)
    - [Run the Application](#run-the-application)
  - [How to Use](#how-to-use)
    - [Example Interaction](#example-interaction)
    - [Modifying the Prompt](#modifying-the-prompt)
  - [Code Structure](#code-structure)
    - [`main.rs`](#mainrs)
    - [`flight_search_tool.rs`](#flight_search_toolrs)
  - [Troubleshooting](#troubleshooting)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- **Natural Language Queries**: Interact with the assistant using plain English.
- **Flight Search**: Find flights between any two airports.
- **Customizable**: Modify the code to add more features or tools.
- **Asynchronous Execution**: Built using asynchronous Rust for efficient performance.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust**: Installed Rust programming language. If not, download and install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **API Keys**:
  - **OpenAI API Key**: Sign up and get your key from [OpenAI API](https://platform.openai.com/account/api-keys).
  - **RapidAPI Key**: Sign up and get your key from [RapidAPI](https://rapidapi.com/hub). We'll use this to access the TripAdvisor Flight Search API.

## Getting Started

Follow these instructions to set up and run the project on your local machine.

### Clone the Repository

Open your terminal and run:

```bash
git clone https://github.com/0xPlaygrounds/awesome-rig.git
cd flight_search_assistant
```

### Set Up Environment Variables

Create a `.env` file in the root directory of the project to store your API keys:

```bash
touch .env
```

Open the `.env` file in your favorite text editor and add the following lines:

```env
OPENAI_API_KEY=your_openai_api_key_here
RAPIDAPI_KEY=your_rapidapi_key_here
```

Replace `your_openai_api_key_here` and `your_rapidapi_key_here` with your actual API keys.

**Note**: Ensure that the `.env` file is added to your `.gitignore` to prevent committing sensitive information.

## Build and Run

### Install Dependencies

Run the following command to download and compile all the dependencies:

```bash
cargo build
```

### Build the Project

To build the project, run:

```bash
cargo build --release
```

This will create an optimized build of the application.

### Run the Application

Execute the application using:

```bash
cargo run
```

You should see output similar to:

```
Agent response:
Here are some flight options:

1. **Airline**: Delta Air Lines
   - **Flight Number**: DL123
   - **Departure**: 2024-11-15T08:00:00-06:00
   - **Arrival**: 2024-11-15T10:45:00-05:00
   - **Duration**: 2 hours 45 minutes
   - **Stops**: Non-stop
   - **Price**: 250.00 USD
   - **Booking URL**: https://www.tripadvisor.com/CheapFlightsPartnerHandoff...

...
```

**Note**: The actual results may vary depending on the API response and the current date.

## How to Use

### Example Interaction

The agent is programmed to respond to natural language prompts. In `main.rs`, the prompt is set as:

```rust
let response = agent
    .prompt("Find me flights from San Antonio (SAT) to London (LHR) on November 15th 2024.")
    .await?;
```

You can modify this prompt to search for flights between different airports or on different dates.

### Modifying the Prompt

To change the interaction, open `src/main.rs` and edit the `prompt` method:

```rust
let response = agent
    .prompt("Your custom prompt here")
    .await?;
```

For example:

```rust
let response = agent
    .prompt("I need a flight from New York (JFK) to Tokyo (HND) on December 20th 2024.")
    .await?;
```

After modifying, save the file and run the application again:

```bash
cargo run
```

## Code Structure

### `main.rs`

This is the entry point of the application. It performs the following tasks:

- Initializes the OpenAI client using your API key.
- Builds the AI agent with a preamble and the `FlightSearchTool`.
- Sends a prompt to the agent.
- Prints the agent's response.

```rust
mod flight_search_tool;

use crate::flight_search_tool::FlightSearchTool;
use dotenv::dotenv;
use rig::completion::Prompt;
use rig::providers::openai;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Build the agent with the FlightSearchTool
    let agent = openai_client
        .agent("gpt-4")
        .preamble("You are a travel assistant that can help users find flights between airports.")
        .tool(FlightSearchTool)
        .build();

    // Send a prompt to the agent
    let response = agent
        .prompt("Find me flights from San Antonio (SAT) to London (LHR) on November 15th 2024.")
        .await?;

    // Print the agent's response
    println!("Agent response:\n{}", response);

    Ok(())
}
```

### `flight_search_tool.rs`

This file defines the `FlightSearchTool`, which interacts with the TripAdvisor Flight Search API to fetch flight information.

Key components:

- **Structs**:
  - `FlightSearchArgs`: Represents the input arguments for the flight search.
  - `FlightOption`: Represents each flight option returned by the API.
- **Error Handling**:
  - `FlightSearchError`: Custom error type to handle various errors that might occur.
- **Implementation**:
  - Implements the `Tool` trait for `FlightSearchTool`.
  - Defines the `definition` and `call` methods required by the trait.
  - The `call` method makes an HTTP request to the API, parses the response, and formats the output.

```rust
use chrono::Utc;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;

// Define the arguments for the flight search
#[derive(Deserialize)]
pub struct FlightSearchArgs {
    source: String,
    destination: String,
    date: Option<String>,
    // Additional optional parameters...
}

// Define the flight option structure
#[derive(Serialize)]
pub struct FlightOption {
    airline: String,
    flight_number: String,
    departure: String,
    arrival: String,
    duration: String,
    stops: usize,
    price: f64,
    currency: String,
    booking_url: String,
}

// Define custom error types
#[derive(Debug, thiserror::Error)]
pub enum FlightSearchError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(String),
    #[error("Invalid response structure")]
    InvalidResponse,
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Missing API key")]
    MissingApiKey,
}

// Implement the Tool trait for FlightSearchTool
pub struct FlightSearchTool;

impl Tool for FlightSearchTool {
    const NAME: &'static str = "search_flights";

    type Args = FlightSearchArgs;
    type Output = String;
    type Error = FlightSearchError;

    // Define the tool
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        // Tool metadata and parameters
    }

    // Implement the call method
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Fetch API key, set defaults, build query params, make API request
        // Parse response and format output
    }
}
```

## Troubleshooting

- **Missing API Keys**: Ensure that your `.env` file contains the correct API keys and that the keys are valid.
- **Dependency Errors**: Run `cargo update` to update dependencies to their latest versions.
- **API Errors**: Check the API usage limits and ensure that your keys have sufficient permissions.

## Contributing

Contributions are welcome! If you'd like to add features, fix bugs, or improve documentation, feel free to open a pull request.

1. Fork the repository.
2. Create a new branch:

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. Make your changes.
4. Commit and push:

   ```bash
   git commit -m "Description of your changes"
   git push origin feature/your-feature-name
   ```

5. Open a pull request on GitHub.

## License

This project is licensed under the [MIT License](LICENSE).

---

*Happy coding! If you have any questions or need further assistance, feel free to open an issue or reach out.*