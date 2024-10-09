# Comprehensive Guide to Rig: Rust Library for LLM-Powered Applications

## 1. Introduction to Rig

Rig is an open-source Rust library designed to simplify the development of applications powered by Large Language Models (LLMs). It provides a unified API for working with different LLM providers, advanced AI workflow support, and flexible abstractions for building complex AI systems.

Key features of Rig include:
- Unified API across multiple LLM providers (e.g., OpenAI, Anthropic, Cohere, Perplexity)
- Support for completion and embedding workflows
- High-level abstractions for agents and RAG systems
- Extensible architecture for custom implementations
- Seamless integration with Rust's ecosystem
- Vector store support, including in-memory and LanceDB options

## 2. Core Concepts

### 2.1 Completion Models

Completion models are the foundation of LLM interactions in Rig. They implement the `CompletionModel` trait, which defines methods for generating completion requests and executing them.

```rust
pub trait CompletionModel: Clone + Send + Sync {
    type Response: Send + Sync;

    fn completion(
        &self,
        request: CompletionRequest,
    ) -> impl std::future::Future<Output = Result<CompletionResponse<Self::Response>, CompletionError>>
           + Send;

    fn completion_request(&self, prompt: &str) -> CompletionRequestBuilder<Self>;
}
```

### 2.2 Embedding Models

Embedding models are used for generating vector representations of text. They implement the `EmbeddingModel` trait:

```rust
pub trait EmbeddingModel: Clone + Sync + Send {
    const MAX_DOCUMENTS: usize;

    fn ndims(&self) -> usize;

    fn embed_documents(
        &self,
        documents: Vec<String>,
    ) -> impl std::future::Future<Output = Result<Vec<Embedding>, EmbeddingError>> + Send;
}
```

### 2.3 Agents

Agents in Rig combine an LLM model with a preamble (system prompt) and a set of tools. They are implemented using the `Agent` struct:

```rust
pub struct Agent<M: CompletionModel> {
    model: M,
    preamble: String,
    static_context: Vec<Document>,
    static_tools: Vec<String>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    additional_params: Option<serde_json::Value>,
    dynamic_context: Vec<(usize, Box<dyn VectorStoreIndexDyn>)>,
    dynamic_tools: Vec<(usize, Box<dyn VectorStoreIndexDyn>)>,
    pub tools: ToolSet,
}
```

### 2.4 Tools

Tools are functionalities that agents can use to perform specific tasks. They implement the `Tool` trait:

```rust
pub trait Tool: Sized + Send + Sync {
    const NAME: &'static str;
    type Error: std::error::Error + Send + Sync + 'static;
    type Args: for<'a> Deserialize<'a> + Send + Sync;
    type Output: Serialize;

    fn name(&self) -> String;
    fn definition(&self, _prompt: String) -> impl Future<Output = ToolDefinition> + Send + Sync;
    fn call(
        &self,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync;
}
```

### 2.5 Vector Stores

Vector stores are used for storing and retrieving embeddings. They implement the `VectorStore` trait:

```rust
pub trait VectorStore: Send + Sync {
    type Q;

    fn add_documents(
        &mut self,
        documents: Vec<DocumentEmbeddings>,
    ) -> impl std::future::Future<Output = Result<(), VectorStoreError>> + Send;

    fn get_document_embeddings(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<DocumentEmbeddings>, VectorStoreError>> + Send;

    // Other methods...
}
```

## 3. Building with Rig

### 3.1 Setting up a Project

To start a new project with Rig, add it to your `Cargo.toml`:

```toml
[dependencies]
rig-core = "0.2.1"
tokio = { version = "1.34.0", features = ["full"] }
```

### 3.2 Creating a Simple Agent

Here's how to create and use a simple agent:

```rust
use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let agent = openai_client
        .agent("gpt-4")
        .preamble("You are a helpful assistant.")
        .build();

    let response = agent.prompt("Explain quantum computing in one sentence.").await?;
    println!("Agent: {}", response);

    Ok(())
}
```

### 3.3 Implementing a Custom Tool

Here's an example of implementing a custom tool:

```rust
use rig::tool::Tool;
use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct AddArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Deserialize, Serialize)]
struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add";
    type Error = MathError;
    type Args = AddArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                },
                "required": ["x", "y"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.x + args.y)
    }
}
```

### 3.4 Creating an Agent with Tools

Here's how to create an agent with custom tools:

```rust
let agent = openai_client.agent("gpt-4")
    .preamble("You are a calculator assistant.")
    .tool(Adder)
    .build();

let response = agent.prompt("Calculate 2 + 3").await?;
println!("Agent: {}", response);
```

### 3.5 Implementing a RAG System

Here's an example of setting up a RAG system with Rig:

```rust
use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::{in_memory_store::InMemoryVectorStore, VectorStore};

let embedding_model = openai_client.embedding_model(openai::TEXT_EMBEDDING_ADA_002);
let mut vector_store = InMemoryVectorStore::default();

let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
    .simple_document("doc1", "Rig is a Rust library for building LLM applications.")
    .simple_document("doc2", "Rig supports OpenAI, Anthropic, Cohere, and Perplexity as LLM providers.")
    .build()
    .await?;

vector_store.add_documents(embeddings).await?;

let rag_agent = openai_client.agent("gpt-4")
    .preamble("You are an assistant that answers questions about Rig.")
    .dynamic_context(1, vector_store.index(embedding_model))
    .build();

let response = rag_agent.prompt("What is Rig?").await?;
println!("RAG Agent: {}", response);
```

## 4. Advanced Features

### 4.1 Customizing Completion Requests

Rig allows for fine-tuning completion requests:

```rust
let response = model.completion_request("Translate to French:")
    .temperature(0.7)
    .max_tokens(50)
    .additional_params(json!({"top_p": 0.9}))
    .send()
    .await?;
```

### 4.2 Batched Embeddings

For efficient embedding generation:

```rust
let embeddings = EmbeddingsBuilder::new(embedding_model)
    .simple_documents(vec![
        ("doc1", "Content 1"),
        ("doc2", "Content 2"),
        // ...
    ])
    .build()
    .await?;
```

### 4.3 Using Different LLM Providers

Rig supports multiple LLM providers. Here's how to use different providers:

```rust
// OpenAI
let openai_client = openai::Client::from_env();
let gpt4_agent = openai_client.agent("gpt-4").build();

// Anthropic
let anthropic_client = anthropic::ClientBuilder::new(&std::env::var("ANTHROPIC_API_KEY")?)
    .build();
let claude_agent = anthropic_client.agent(anthropic::CLAUDE_3_5_SONNET).build();

// Cohere
let cohere_client = cohere::Client::new(&std::env::var("COHERE_API_KEY")?);
let command_agent = cohere_client.agent("command").build();

// Perplexity
let perplexity_client = perplexity::Client::new(&std::env::var("PERPLEXITY_API_KEY")?);
let llama_agent = perplexity_client.agent(perplexity::LLAMA_3_1_70B_INSTRUCT).build();
```

### 4.4 Using LanceDB for Vector Storage

Here's an example of using LanceDB with Rig:

```rust
use rig_lancedb::{LanceDbVectorStore, SearchParams};

let db = lancedb::connect("data/lancedb-store").execute().await?;

let table = db.create_table(
    "rig_docs",
    RecordBatchIterator::new(vec![record_batch], Arc::new(rig_lancedb::schema(model.ndims()))),
).execute().await?;

let search_params = SearchParams::default();
let vector_store = LanceDbVectorStore::new(table, model, "id", search_params).await?;

// Use vector_store in your RAG system...
```

## 5. Best Practices and Tips

1. **Error Handling**: Use Rig's error types for robust error handling.
2. **Asynchronous Programming**: Leverage Rust's async features with Rig for efficient I/O operations.
3. **Modular Design**: Break down complex AI workflows into reusable tools and agents.
4. **Security**: Always use environment variables or secure vaults for API keys.
5. **Testing**: Write unit tests for custom tools and mock LLM responses for consistent testing.
6. **Model Selection**: Choose appropriate models based on your task complexity and performance requirements.
7. **Prompt Engineering**: Craft clear and specific prompts, utilizing the `preamble` method for setting agent behavior.
8. **Vector Store Usage**: Use vector stores efficiently, generating embeddings once and reusing them when possible.

## 6. Troubleshooting Common Issues

1. **API Rate Limiting**: Implement retries with exponential backoff for API calls.
2. **Memory Usage**: For large document sets, consider using LanceDB or other database-backed vector stores instead of in-memory solutions.
3. **Compatibility**: Ensure you're using compatible versions of Rig and its dependencies.
4. **Embedding Dimensions**: Make sure to use the correct number of dimensions when working with embeddings and vector stores.

## 7. Community and Support

- GitHub Repository: https://github.com/0xPlaygrounds/rig
- Documentation: https://docs.rs/rig-core/latest/rig/
- Discord Community: [Join here] (replace with actual Discord link when available)

## 8. Future Roadmap

- Support for more LLM providers
- Enhanced performance optimizations
- Advanced AI workflow templates
- Ecosystem growth with additional tools and libraries
- Improved documentation and examples

This comprehensive guide covers the core concepts, usage patterns, and advanced features of Rig. It provides a solid foundation for developing LLM-powered applications using Rig and serves as a reference for both beginners and experienced users of the library.