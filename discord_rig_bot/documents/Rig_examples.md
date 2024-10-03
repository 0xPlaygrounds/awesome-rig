1. Building a Simple Agent

```rust
use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    
    let comedian_agent = openai_client
        .agent("gpt-4")
        .preamble("You are a comedian here to entertain the user using humor and jokes.")
        .build();

    let response = comedian_agent.prompt("Tell me a joke about programming.").await?;
    println!("{}", response);

    Ok(())
}
```

2. Creating a Custom Tool

```rust
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct WeatherArgs {
    city: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Weather API error")]
struct WeatherError;

#[derive(Serialize)]
struct WeatherInfo {
    temperature: f32,
    condition: String,
}

struct WeatherTool;

impl Tool for WeatherTool {
    const NAME: &'static str = "get_weather";
    type Error = WeatherError;
    type Args = WeatherArgs;
    type Output = WeatherInfo;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get current weather for a city".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "The city to get weather for"
                    }
                },
                "required": ["city"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // In a real implementation, you would call a weather API here
        Ok(WeatherInfo {
            temperature: 22.5,
            condition: "Sunny".to_string(),
        })
    }
}
```

3. Using Different Models (OpenAI and Cohere)

```rust
use rig::{completion::Prompt, providers::{openai, cohere}};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let cohere_client = cohere::Client::new(&std::env::var("COHERE_API_KEY")?);

    let gpt4 = openai_client.model("gpt-4").build();
    let command = cohere_client.model("command").build();

    let gpt4_response = gpt4.prompt("Explain quantum computing").await?;
    let command_response = command.prompt("Explain quantum computing").await?;

    println!("GPT-4: {}", gpt4_response);
    println!("Cohere Command: {}", command_response);

    Ok(())
}
```

4. Chaining Agents

```rust
use rig::{completion::{Chat, Message}, providers::openai};

struct TranslatorAgent {
    translator: Agent<openai::CompletionModel>,
    responder: Agent<openai::CompletionModel>,
}

impl TranslatorAgent {
    fn new(openai_client: &openai::Client) -> Self {
        Self {
            translator: openai_client.agent("gpt-4")
                .preamble("You are a translator. Translate the input to English.")
                .build(),
            responder: openai_client.agent("gpt-4")
                .preamble("You are a helpful assistant. Respond to the user's question.")
                .build(),
        }
    }
}

impl Chat for TranslatorAgent {
    async fn chat(&self, prompt: &str, chat_history: Vec<Message>) -> Result<String, PromptError> {
        let translated = self.translator.chat(prompt, vec![]).await?;
        self.responder.chat(&translated, chat_history).await
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let agent = TranslatorAgent::new(&openai_client);

    let response = agent.chat("Bonjour, comment ça va?", vec![]).await?;
    println!("Response: {}", response);

    Ok(())
}
```

5. RAG Agent with Dynamic Tools

```rust
use rig::{
    providers::openai,
    embeddings::EmbeddingsBuilder,
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
    tool::{Tool, ToolSet},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

    // Create vector store and add documents
    let mut vector_store = InMemoryVectorStore::default();
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc1", "Rig is a Rust library for building LLM applications.")
        .simple_document("doc2", "Rig supports OpenAI and Cohere as LLM providers.")
        .build()
        .await?;
    vector_store.add_documents(embeddings).await?;

    // Create dynamic tools
    let toolset = ToolSet::builder()
        .dynamic_tool(WeatherTool)
        // Add more dynamic tools here
        .build();

    // Create RAG agent with dynamic tools
    let rag_agent = openai_client.agent("gpt-4")
        .preamble("You are an assistant that can answer questions about Rig and check the weather.")
        .dynamic_context(2, vector_store.index(embedding_model.clone()))
        .dynamic_tools(1, vector_store.index(embedding_model), toolset)
        .build();

    let response = rag_agent.prompt("What is Rig and what's the weather like in New York?").await?;
    println!("RAG Agent: {}", response);

    Ok(())
}
```

6. Using Extractors

```rust
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Person {
    name: String,
    age: u8,
    occupation: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    
    let extractor = openai_client.extractor::<Person>("gpt-4").build();

    let text = "John Doe is a 30-year-old software engineer.";
    let person = extractor.extract(text).await?;

    println!("Extracted person: {:?}", person);

    Ok(())
}
```

7. Text Classification System

```rust
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct SentimentClassification {
    sentiment: Sentiment,
    confidence: f32,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    
    let classifier = openai_client
        .extractor::<SentimentClassification>("gpt-4")
        .preamble("Classify the sentiment of the given text as Positive, Negative, or Neutral.")
        .build();

    let text = "I love using Rig for building LLM applications!";
    let classification = classifier.extract(text).await?;

    println!("Sentiment: {:?}, Confidence: {}", classification.sentiment, classification.confidence);

    Ok(())
}
```

8. Multi-Agent System

```rust
use rig::{completion::{Chat, Message}, providers::openai};

struct DebateAgents {
    agent_a: Agent<openai::CompletionModel>,
    agent_b: Agent<openai::CompletionModel>,
}

impl DebateAgents {
    fn new(openai_client: &openai::Client) -> Self {
        Self {
            agent_a: openai_client.agent("gpt-4")
                .preamble("You are debating in favor of renewable energy.")
                .build(),
            agent_b: openai_client.agent("gpt-4")
                .preamble("You are debating in favor of nuclear energy.")
                .build(),
        }
    }

    async fn debate(&self, rounds: usize) -> Result<(), anyhow::Error> {
        let mut history_a = vec![];
        let mut history_b = vec![];

        for i in 0..rounds {
            println!("Round {}:", i + 1);
            
            let response_a = self.agent_a.chat("Present your argument", history_a.clone()).await?;
            println!("Agent A: {}", response_a);
            history_b.push(Message { role: "user".into(), content: response_a });

            let response_b = self.agent_b.chat("Respond to the argument", history_b.clone()).await?;
            println!("Agent B: {}", response_b);
            history_a.push(Message { role: "user".into(), content: response_b });
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let debate = DebateAgents::new(&openai_client);
    debate.debate(3).await?;
    Ok(())
}
```

9. Vector Search with Cohere

```rust
use rig::{
    providers::cohere,
    embeddings::EmbeddingsBuilder,
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore, VectorStoreIndex},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cohere_client = cohere::Client::new(&std::env::var("COHERE_API_KEY")?);
    
    let document_model = cohere_client.embedding_model("embed-english-v3.0", "search_document");
    let search_model = cohere_client.embedding_model("embed-english-v3.0", "search_query");

    let mut vector_store = InMemoryVectorStore::default();

    let embeddings = EmbeddingsBuilder::new(document_model)
        .simple_document("doc1", "Rig is a Rust library for building LLM applications.")
        .simple_document("doc2", "Rig supports various LLM providers and vector stores.")
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    let index = vector_store.index(search_model);

    let results = index.top_n_from_query("What is Rig?", 1).await?;
    
    for (score, doc) in results {
        println!("Score: {}, Document: {}", score, doc.document);
    }

    Ok(())
}
```

10. Cohere Connectors

```rust
use rig::{completion::Completion, providers::cohere::Client as CohereClient};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cohere_client = CohereClient::new(&std::env::var("COHERE_API_KEY")?);

    let agent = cohere_client
        .agent("command-r")
        .temperature(0.0)
        .additional_params(json!({
            "connectors": [{"id":"web-search", "options":{"site": "https://docs.rs/rig-core"}}]
        }))
        .build();

    let response = agent
        .completion("What are the main features of Rig?", vec![])
        .await?
        .additional_params(json!({
            "connectors": [{"id":"web-search", "options":{"site": "https://docs.rs/rig-core"}}]
        }))
        .send()
        .await?;

    println!("Response: {:?}", response.choice);
    println!("Citations: {:?}", response.raw_response.citations);

    Ok(())
}
```

11. Calculator Chatbot

```rust
use rig::{
    cli_chatbot::cli_chatbot,
    completion::ToolDefinition,
    providers::openai::Client,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct CalculatorArgs {
    x: f64,
    y: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Deserialize, Serialize)]
struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculate";
    type Error = MathError;
    type Args = CalculatorArgs;
    type Output = f64;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Perform basic arithmetic operations".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "First number"
                    },
                    "y": {
                        "type": "number",
                        "description": "Second number"
                    },
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"],
                        "description": "Arithmetic operation to perform"
                    }
                },
                "required": ["x", "y", "operation"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.operation.as_str() {
            "add" => Ok(args.x + args.y),
            "subtract" => Ok(args.x - args.y),
            "multiply" => Ok(args.x * args.y),
            "divide" => {
                if args.y == 0.0 {
                    Err(MathError)
                } else {
                    Ok(args.x / args.y)
                }
            },
            _ => Err(MathError),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = Client::from_env();

    let calculator_agent = openai_client
        .agent("gpt-4")
        .preamble("You are a calculator assistant. Use the calculate tool to perform arithmetic operations.")
        .tool(Calculator)
        .build();

    cli_chatbot(calculator_agent).await?;

    Ok(())
}
```

