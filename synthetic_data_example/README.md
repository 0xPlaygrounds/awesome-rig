# Synthetic Data Generation with [Rig](https://github.com/0xPlaygrounds/rig)

This example showcases how to leverage [Rig](https://github.com/0xPlaygrounds/rig), a powerful Rust library for building LLM-powered applications, to generate realistic synthetic data based on a given schema. Whether you're new to Rig or looking to explore its capabilities, this example provides an excellent starting point for understanding how to work with AI-powered data generation.

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Create a new Rust project:
   ```
   cargo new rig-synthetic-data
   cd rig-synthetic-data
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig-core = "0.1.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   tokio = { version = "1.0", features = ["full"] }
   ```

3. Set your OpenAI API key as an environment variable:
   ```
   export OPENAI_API_KEY=your_api_key_here
   ```

### Code Overview

The main components of this example are:

1. A custom data structure (`PersonData`) for representing our synthetic data.
2. OpenAI client initialization.
3. A data generator setup using the GPT-4 model.
4. A schema and instructions for data generation.
5. The data generation process and result handling.

### Running the Example

1. Copy the provided code into your `src/main.rs` file.
2. Run the example using:
   ```
   cargo run
   ```

### Customization

Feel free to modify the `PersonData` struct or adjust the schema and instructions to generate different types of data. You can also experiment with different OpenAI models by changing the model name in the data generator setup.

### Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).
