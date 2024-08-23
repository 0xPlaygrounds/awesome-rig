# Text Classification with Rig

This example showcases how to use Rig, a powerful Rust library for building LLM-powered applications, to classify text into predefined categories. Whether you're new to Rig or looking to explore its capabilities, this example provides an excellent starting point for understanding how to work with custom data structures and AI-powered classification.

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Create a new Rust project:
   ```
   cargo new rig-text-classification
   cd rig-text-classification
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig = "0.1.0"
   serde = { version = "1.0", features = ["derive"] }
   schemars = "0.8"
   tokio = { version = "1.0", features = ["full"] }
   ```

3. Set your OpenAI API key as an environment variable:
   ```
   export OPENAI_API_KEY=your_api_key_here
   ```

### Code Overview

The main components of this example are:

1. Custom data structures (`Category` enum and `ClassificationResult` struct) for representing classification results.
2. An OpenAI client initialization.
3. A classifier setup using the GPT-4 model.
4. A set of sample texts for classification.
5. The classification process and result handling.

### Running the Example

1. Copy the provided code into your `src/main.rs` file.
2. Run the example using:
   ```
   cargo run
   ```

### Customization

Feel free to modify the `sample_texts` or adjust the `Category` enum to suit your specific use case. You can also experiment with different OpenAI models by changing the model name in the classifier setup.

### Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).

