# Rig code samples

1. Building a simple agent with Rig:
```rust
use std::env;

use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = providers::openai::Client::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
    );

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("gpt-4o")
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Entertain me!").await?;
    println!("{}", response);

    Ok(())
}
```

2. Building an agent with context with Rig:
```rust
use std::env;

use rig::{agent::AgentBuilder, completion::Prompt, providers::cohere};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI and Cohere clients
    // let openai_client = openai::Client::new(&env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));
    let cohere_client =
        cohere::Client::new(&env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set"));

    // let model = openai_client.completion_model("gpt-4");
    let model = cohere_client.completion_model("command-r");

    // Create an agent with multiple context documents
    let agent = AgentBuilder::new(model)
        .context("Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .context("Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .context("Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build();

    // Prompt the agent and print the response
    let response = agent.prompt("What does \"glarb-glarb\" mean?").await?;

    println!("{}", response);

    Ok(())
}
```

3. Building an agent with tools with Rig:
```rust
use anyhow::Result;
use rig::{
    completion::{Prompt, ToolDefinition},
    providers,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Deserialize)]
struct OperationArgs {
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
    type Args = OperationArgs;
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
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y;
        Ok(result)
    }
}

#[derive(Deserialize, Serialize)]
struct Subtract;
impl Tool for Subtract {
    const NAME: &'static str = "subtract";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "subtract",
            "description": "Subtract y from x (i.e.: x - y)",
            "parameters": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The number to substract from"
                    },
                    "y": {
                        "type": "number",
                        "description": "The number to substract"
                    }
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x - args.y;
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = providers::openai::Client::new(&openai_api_key);

    // Create agent with a single context prompt and two tools
    let gpt4_calculator_agent = openai_client
        .agent("gpt-4")
        .context("You are a calculator here to help the user perform arithmetic operations.")
        .tool(Adder)
        .tool(Subtract)
        .build();

    // Create OpenAI client
    let cohere_api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
    let cohere_client = providers::cohere::Client::new(&cohere_api_key);

    // Create agent with a single context prompt and two tools
    let coral_calculator_agent = cohere_client
        .agent("command-r")
        .preamble("You are a calculator here to help the user perform arithmetic operations.")
        .tool(Adder)
        .tool(Subtract)
        .build();

    // Prompt the agent and print the response
    println!("Calculate 2 - 5");
    println!(
        "GPT-4: {}",
        gpt4_calculator_agent.prompt("Calculate 2 - 5").await?
    );
    println!(
        "Coral: {}",
        coral_calculator_agent.prompt("Calculate 2 - 5").await?
    );

    Ok(())
}
```

4. Building an Anthropic agent with Rig:
```rust
use std::env;

use rig::{
    completion::Prompt,
    providers::anthropic::{self, CLAUDE_3_5_SONNET},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = anthropic::ClientBuilder::new(
        &env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
    )
    .build();

    // Create agent with a single context prompt
    let agent = client
        .agent(CLAUDE_3_5_SONNET)
        .preamble("Be precise and concise.")
        .temperature(0.5)
        .max_tokens(8192)
        .build();

    // Prompt the agent and print the response
    let response = agent
        .prompt("When and where and what type is the next solar eclipse?")
        .await?;
    println!("{}", response);

    Ok(())
}
```

5. Building a calculator chatbot with Rig:
```rust
use anyhow::Result;
use rig::{
    cli_chatbot::cli_chatbot,
    completion::ToolDefinition,
    embeddings::EmbeddingsBuilder,
    providers::openai::Client,
    tool::{Tool, ToolEmbedding, ToolSet},
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
struct InitError;

#[derive(Deserialize, Serialize)]
struct Add;
impl Tool for Add {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "add",
            "description": "Add x and y together",
            "parameters": {
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
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Add {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Add)
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Add x and y together".into()]
    }

    fn context(&self) -> Self::Context {}
}

#[derive(Deserialize, Serialize)]
struct Subtract;
impl Tool for Subtract {
    const NAME: &'static str = "subtract";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "subtract",
            "description": "Subtract y from x (i.e.: x - y)",
            "parameters": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The number to substract from"
                    },
                    "y": {
                        "type": "number",
                        "description": "The number to substract"
                    }
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x - args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Subtract {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Subtract)
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Subtract y from x (i.e.: x - y)".into()]
    }

    fn context(&self) -> Self::Context {}
}

struct Multiply;
impl Tool for Multiply {
    const NAME: &'static str = "multiply";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "multiply",
            "description": "Compute the product of x and y (i.e.: x * y)",
            "parameters": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first factor in the product"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second factor in the product"
                    }
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x * args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Multiply {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Multiply)
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Compute the product of x and y (i.e.: x * y)".into()]
    }

    fn context(&self) -> Self::Context {}
}

struct Divide;
impl Tool for Divide {
    const NAME: &'static str = "divide";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "divide",
            "description": "Compute the Quotient of x and y (i.e.: x / y). Useful for ratios.",
            "parameters": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The Dividend of the division. The number being divided"
                    },
                    "y": {
                        "type": "number",
                        "description": "The Divisor of the division. The number by which the dividend is being divided"
                    }
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x / args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Divide {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Divide)
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Compute the Quotient of x and y (i.e.: x / y). Useful for ratios.".into()]
    }

    fn context(&self) -> Self::Context {}
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Client::new(&openai_api_key);

    // Create dynamic tools embeddings
    let toolset = ToolSet::builder()
        .dynamic_tool(Add)
        .dynamic_tool(Subtract)
        .dynamic_tool(Multiply)
        .dynamic_tool(Divide)
        .build();

    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .tools(&toolset)?
        .build()
        .await?;

    let mut store = InMemoryVectorStore::default();
    store.add_documents(embeddings).await?;
    let index = store.index(embedding_model);

    // Create RAG agent with a single context prompt and a dynamic tool source
    let calculator_rag = openai_client
        .agent("gpt-4")
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform arithmetic operations.
            Follow these instructions closely. 
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context. 
            3. This is very important: never perform the operation yourself and never give me the direct result. 
            Always respond with the name of the tool that should be used and the appropriate inputs
            in the following format:
            Tool: <tool name>
            Inputs: <list of inputs>
            "
        )
        // Add a dynamic tool source with a sample rate of 1 (i.e.: only
        // 1 additional tool will be added to prompts)
        .dynamic_tools(4, index, toolset)
        .build();

    // Prompt the agent and print the response

    cli_chatbot(calculator_rag).await?;

    Ok(())
}
```

6. Building a cohere connector with Rig:
```rust
use std::env;

use rig::{
    completion::{Completion, Prompt},
    providers::cohere::Client as CohereClient,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create Cohere client
    let cohere_api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
    let cohere_client = CohereClient::new(&cohere_api_key);

    let klimadao_agent = cohere_client
        .agent("command-r")
        .temperature(0.0)
        .additional_params(json!({
            "connectors": [{"id":"web-search", "options":{"site": "https://docs.klimadao.finance"}}]
        }))
        .build();

    // Prompt the model and print the response
    // We use `prompt` to get a simple response from the model as a String
    let response = klimadao_agent.prompt("Tell me about BCT tokens?").await?;

    println!("\n\nCoral: {:?}", response);

    // Prompt the model and get the citations
    // We use `completion` to allow use to customize the request further and
    // get a more detailed response from the model.
    // Here the response is of type CompletionResponse<cohere::CompletionResponse>
    // which contains `choice` (Message or ToolCall) as well as `raw_response`,
    // the underlying providers' raw response.
    let response = klimadao_agent
        .completion("Tell me about BCT tokens?", vec![])
        .await?
        .additional_params(json!({
            "connectors": [{"id":"web-search", "options":{"site": "https://docs.klimadao.finance"}}]
        }))
        .send()
        .await?;

    println!(
        "\n\nCoral: {:?}\n\nCitations:\n{:?}",
        response.choice, response.raw_response.citations
    );

    Ok(())
}
```

7. Building debate agents with Rig:
```rust
use std::env;

use anyhow::Result;
use rig::{
    agent::Agent,
    completion::{Chat, Message},
    providers::{cohere, openai},
};

struct Debater {
    gpt_4: Agent<openai::CompletionModel>,
    coral: Agent<cohere::CompletionModel>,
}

impl Debater {
    fn new(position_a: &str, position_b: &str) -> Self {
        let openai_client =
            openai::Client::new(&env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));
        let cohere_client =
            cohere::Client::new(&env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set"));

        Self {
            gpt_4: openai_client.agent("gpt-4").preamble(position_a).build(),
            coral: cohere_client
                .agent("command-r")
                .preamble(position_b)
                .build(),
        }
    }

    async fn rounds(&self, n: usize) -> Result<()> {
        let mut history_a: Vec<Message> = vec![];
        let mut history_b: Vec<Message> = vec![];

        let mut last_resp_b: Option<String> = None;

        for _ in 0..n {
            let prompt_a = if let Some(msg_b) = &last_resp_b {
                msg_b.clone()
            } else {
                "Plead your case!".into()
            };

            let resp_a = self.gpt_4.chat(&prompt_a, history_a.clone()).await?;
            println!("GPT-4:\n{}", resp_a);
            history_a.push(Message {
                role: "user".into(),
                content: prompt_a.clone(),
            });
            history_a.push(Message {
                role: "assistant".into(),
                content: resp_a.clone(),
            });
            println!("================================================================");

            let resp_b = self.coral.chat(&resp_a, history_b.clone()).await?;
            println!("Coral:\n{}", resp_b);
            println!("================================================================");

            history_b.push(Message {
                role: "user".into(),
                content: resp_a.clone(),
            });
            history_b.push(Message {
                role: "assistant".into(),
                content: resp_b.clone(),
            });

            last_resp_b = Some(resp_b)
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create model
    let debator = Debater::new(
        "\
        You believe that religion is a useful concept. \
        This could be for security, financial, ethical, philosophical, metaphysical, religious or any kind of other reason. \
        You choose what your arguments are. \
        I will argue against you and you must rebuke me and try to convince me that I am wrong. \
        Make your statements short and concise. \
        ",
        "\
        You believe that religion is a harmful concept. \
        This could be for security, financial, ethical, philosophical, metaphysical, religious or any kind of other reason. \
        You choose what your arguments are. \
        I will argue against you and you must rebuke me and try to convince me that I am wrong. \
        Make your statements short and concise. \
        ",
    );

    // Run the debate for 4 rounds
    debator.rounds(4).await?;

    Ok(())
}
```

8. Building extractor with Rig:
```rust
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
/// A record representing a person
struct Person {
    /// The person's first name, if provided (null otherwise)
    pub first_name: Option<String>,
    /// The person's last name, if provided (null otherwise)
    pub last_name: Option<String>,
    /// The person's job, if provided (null otherwise)
    pub job: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_client = openai::Client::from_env();

    // Create extractor
    let data_extractor = openai_client.extractor::<Person>("gpt-4").build();

    let person = data_extractor
        .extract("Hello my name is John Doe! I am a software engineer.")
        .await?;

    println!("GPT-4: {}", serde_json::to_string_pretty(&person).unwrap());

    Ok(())
}
```

9. Building multi agents with Rig:
```rust
use std::env;

use rig::{
    agent::{Agent, AgentBuilder},
    cli_chatbot::cli_chatbot,
    completion::{Chat, CompletionModel, Message, PromptError},
    providers::openai::Client as OpenAIClient,
};

/// Represents a multi agent application that consists of two components:
/// an agent specialized in translating prompt into english and a simple GPT-4 model.
/// When prompted, the application will use the translator agent to translate the
/// prompt in english, before answering it with GPT-4. The answer in english is returned.
struct EnglishTranslator<M: CompletionModel> {
    translator_agent: Agent<M>,
    gpt4: Agent<M>,
}

impl<M: CompletionModel> EnglishTranslator<M> {
    fn new(model: M) -> Self {
        Self {
            // Create the translator agent
            translator_agent: AgentBuilder::new(model.clone())
                .preamble("\
                    You are a translator assistant that will translate any input text into english. \
                    If the text is already in english, simply respond with the original text but fix any mistakes (grammar, syntax, etc.). \
                ")
                .build(),

            // Create the GPT4 model
            gpt4: AgentBuilder::new(model).build()
        }
    }
}

impl<M: CompletionModel> Chat for EnglishTranslator<M> {
    async fn chat(&self, prompt: &str, chat_history: Vec<Message>) -> Result<String, PromptError> {
        // Translate the prompt using the translator agent
        let translated_prompt = self
            .translator_agent
            .chat(prompt, chat_history.clone())
            .await?;

        println!("Translated prompt: {}", translated_prompt);

        // Answer the prompt using gpt4
        self.gpt4.chat(&translated_prompt, chat_history).await
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = OpenAIClient::new(&openai_api_key);
    let model = openai_client.completion_model("gpt-4");

    // Create OpenAI client
    // let cohere_api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
    // let cohere_client = CohereClient::new(&cohere_api_key);
    // let model = cohere_client.completion_model("command-r");

    // Create model
    let translator = EnglishTranslator::new(model);

    // Spin up a chatbot using the agent
    cli_chatbot(translator).await?;

    Ok(())
}
```

10. Building perplexity agent with Rig:
```rust
use std::env;

use rig::{
    completion::Prompt,
    providers::{self, perplexity::LLAMA_3_1_70B_INSTRUCT},
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = providers::perplexity::Client::new(
        &env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY not set"),
    );

    // Create agent with a single context prompt
    let agent = client
        .agent(LLAMA_3_1_70B_INSTRUCT)
        .preamble("Be precise and concise.")
        .temperature(0.5)
        .additional_params(json!({
            "return_related_questions": true,
            "return_images": true
        }))
        .build();

    // Prompt the agent and print the response
    let response = agent
        .prompt("When and where and what type is the next solar eclipse?")
        .await?;
    println!("{}", response);

    Ok(())
}
```

11. Building RAG Agent with Rig:
```rust
use std::env;

use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::openai::{Client, TEXT_EMBEDDING_ADA_002},
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Client::new(&openai_api_key);

    let embedding_model = openai_client.embedding_model(TEXT_EMBEDDING_ADA_002);

    // Create vector store, compute embeddings and load them in the store
    let mut vector_store = InMemoryVectorStore::default();

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc0", "Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .simple_document("doc1", "Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .simple_document("doc2", "Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    // Create vector store index
    let index = vector_store.index(embedding_model);

    let rag_agent = openai_client.agent("gpt-4")
        .preamble("
            You are a dictionary assistant here to assist the user in understanding the meaning of words.
            You will find additional non-standard word definitions that could be useful below.
        ")
        .dynamic_context(1, index)
        .build();

    // Prompt the agent and print the response
    let response = rag_agent.prompt("What does \"glarb-glarb\" mean?").await?;

    println!("{}", response);

    Ok(())
}
```

12. Building RAG agent with dynamics tools with Rig:
```rust
use anyhow::Result;
use rig::{
    completion::{Prompt, ToolDefinition},
    embeddings::EmbeddingsBuilder,
    providers::openai::Client,
    tool::{Tool, ToolEmbedding, ToolSet},
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct InitError;

#[derive(Deserialize, Serialize)]
struct Add;

impl Tool for Add {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "add",
            "description": "Add x and y together",
            "parameters": {
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
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Add {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Add)
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Add x and y together".into()]
    }

    fn context(&self) -> Self::Context {}
}

#[derive(Deserialize, Serialize)]
struct Subtract;

impl Tool for Subtract {
    const NAME: &'static str = "subtract";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "subtract",
            "description": "Subtract y from x (i.e.: x - y)",
            "parameters": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The number to substract from"
                    },
                    "y": {
                        "type": "number",
                        "description": "The number to substract"
                    }
                }
            }
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x - args.y;
        Ok(result)
    }
}

impl ToolEmbedding for Subtract {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Subtract)
    }

    fn context(&self) -> Self::Context {}

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Subtract y from x (i.e.: x - y)".into()]
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Client::new(&openai_api_key);

    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

    // Create vector store, compute tool embeddings and load them in the store
    let mut vector_store = InMemoryVectorStore::default();

    let toolset = ToolSet::builder()
        .dynamic_tool(Add)
        .dynamic_tool(Subtract)
        .build();

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .tools(&toolset)?
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    // Create vector store index
    let index = vector_store.index(embedding_model);

    // Create RAG agent with a single context prompt and a dynamic tool source
    let calculator_rag = openai_client
        .agent("gpt-4")
        .preamble("You are a calculator here to help the user perform arithmetic operations.")
        // Add a dynamic tool source with a sample rate of 1 (i.e.: only
        // 1 additional tool will be added to prompts)
        .dynamic_tools(1, index, toolset)
        .build();

    // Prompt the agent and print the response
    let response = calculator_rag.prompt("Calculate 3 - 7").await?;
    println!("{}", response);

    Ok(())
}
```

13. Building sentiment classifiers with Rig:
```rust
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
/// An enum representing the sentiment of a document
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct DocumentSentiment {
    /// The sentiment of the document
    sentiment: Sentiment,
}

#[tokio::main]
async fn main() {
    // Create OpenAI client
    let openai_client = openai::Client::from_env();

    // Create extractor
    let data_extractor = openai_client
        .extractor::<DocumentSentiment>("gpt-4")
        .build();

    let sentiment = data_extractor
        .extract("I am happy")
        .await
        .expect("Failed to extract sentiment");

    println!("GPT-4: {:?}", sentiment);
}
```

14. Simple vector search with Rig:
```rust
use std::env;

use rig::{
    embeddings::{DocumentEmbeddings, EmbeddingsBuilder},
    providers::openai::Client,
    vector_store::{in_memory_store::InMemoryVectorIndex, VectorStoreIndex},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Client::new(&openai_api_key);

    let model = openai_client.embedding_model("text-embedding-ada-002");

    let embeddings = EmbeddingsBuilder::new(model.clone())
        .simple_document("doc0", "Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .simple_document("doc1", "Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .simple_document("doc2", "Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build()
        .await?;

    let index = InMemoryVectorIndex::from_embeddings(model, embeddings).await?;

    let results = index
        .top_n::<DocumentEmbeddings>("What is a linglingdong?", 1)
        .await?
        .into_iter()
        .map(|(score, id, doc)| (score, id, doc.document))
        .collect::<Vec<_>>();

    println!("Results: {:?}", results);

    let id_results = index
        .top_n_ids("What is a linglingdong?", 1)
        .await?
        .into_iter()
        .map(|(score, id)| (score, id))
        .collect::<Vec<_>>();

    println!("ID results: {:?}", id_results);

    Ok(())
}
```

15. Building cohere vector search with Rig:
```rust
use std::env;

use rig::{
    embeddings::{DocumentEmbeddings, EmbeddingsBuilder},
    providers::cohere::Client,
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore, VectorStoreIndex},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create Cohere client
    let cohere_api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
    let cohere_client = Client::new(&cohere_api_key);

    let document_model = cohere_client.embedding_model("embed-english-v3.0", "search_document");
    let search_model = cohere_client.embedding_model("embed-english-v3.0", "search_query");

    let mut vector_store = InMemoryVectorStore::default();

    let embeddings = EmbeddingsBuilder::new(document_model)
        .simple_document("doc0", "Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .simple_document("doc1", "Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .simple_document("doc2", "Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    let index = vector_store.index(search_model);

    let results = index
        .top_n::<DocumentEmbeddings>("What is a linglingdong?", 1)
        .await?
        .into_iter()
        .map(|(score, id, doc)| (score, id, doc.document))
        .collect::<Vec<_>>();

    println!("Results: {:?}", results);

    Ok(())
}
```
