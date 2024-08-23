# [Rig](https://github.com/0xPlaygrounds/rig)-Powered Tic-Tac-Toe Game

This project demonstrates how to use [Rig](https://github.com/0xPlaygrounds/rig), a powerful Rust library for building LLM-powered applications, to create an AI opponent in a classic game of Tic-Tac-Toe. Whether you're new to Rig or looking to explore AI integration in game development, this example provides an excellent starting point.

### What is [Rig](https://github.com/0xPlaygrounds/rig)?

Rig is a Rust library that simplifies the process of integrating large language models (LLMs) into your applications. It provides an easy-to-use interface for interacting with AI models, allowing developers to focus on their application logic rather than the intricacies of AI API interactions.

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Create a new Rust project:
   ```
   cargo new rig-tictactoe
   cd rig-tictactoe
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig-core = "0.1.0"
   serde = { version = "1.0.193", features = ["derive"] }
   tokio = { version = "1.0", features = ["full"] }
   ```

3. Set your OpenAI API key as an environment variable:
   ```
   export OPENAI_API_KEY=your_api_key_here
   ```

### Code Overview

The main components of this example are:

1. Game state representation (`Player` enum and `Board` struct)
2. Game logic (move validation, win checking, board visualization)
3. AI integration using Rig
4. Main game loop with turn alternation between human and AI

### Running the Game

1. Copy the provided code into your `src/main.rs` file.
2. Run the game using:
   ```
   cargo run
   ```

### Key Concepts

1. **AI Integration**: We use Rig to create an AI player that can understand the game state and make moves:
   ```rust
   let ai_player = openai_client.model("gpt-3.5-turbo").build();
   ```

2. **Prompt Engineering**: We construct prompts that describe the game state and expected response format:
   ```rust
   let prompt = format!(
       "You are playing Tic-Tac-Toe as O. Here's the current board state:\n{}\nWhat's your next move? Respond with just the number (1-9) of the position you want to play.",
       board.to_string()
   );
   ```

3. **Response Parsing**: We parse the AI's responses to extract valid moves:
   ```rust
   fn parse_ai_response(response: &str) -> Result<usize, String> {
       // Parsing logic here
   }
   ```

4. **Error Handling**: We use Rust's `Result` type for robust error handling throughout the game.

5. **Asynchronous Operations**: We use `tokio` for asynchronous execution when interacting with the AI.

### Customization

Feel free to modify the game logic, board visualization, or AI prompts to experiment with different game mechanics or AI behaviors. You could also try using different AI models or adjusting the temperature setting for varied AI responses.

### Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).