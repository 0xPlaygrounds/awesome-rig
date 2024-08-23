# Entity Extraction with [Rig](https://github.com/0xPlaygrounds/rig)

This example demonstrates how to leverage [Rig](https://github.com/0xPlaygrounds/rig), a Rust library for building LLM-powered applications, to extract named entities from text. Whether you're new to Rig or looking to explore its capabilities, this example provides a great starting point for understanding how to work with custom data structures and AI-powered extraction.

## Prerequisites
Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at OpenAI's website.

## Setup

- Create a new Rust project: 
  - `cargo new rig-entity-extraction`
  - `cd rig-entity-extraction`

- Add the following dependencies to your `Cargo.toml`:
```
[dependencies]
rig-core = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = "0.8"
tokio = { version = "1.0", features = ["full"] }
```

- Set your OpenAI API key as an environment variable: 
  - `export OPENAI_API_KEY=your_api_key_here`


## Code Overview

The main components of this example are:

- Custom data structures (EntityType, Entity, ExtractedEntities) for representing extracted entities.
- An OpenAI client initialization.
- An extractor setup using GPT-4 model.
- A sample text for entity extraction.
- The extraction process and result handling.

## Running the Example

- Copy the provided code into your src/main.rs file.
- Run the example using: `cargo run`


## Customization

Feel free to modify the `sample_text` or adjust the `EntityType` enum to suit your specific use case. You can also experiment with different OpenAI models by changing the model name in the extractor setup.


## Troubleshooting
If you encounter any issues:

- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).