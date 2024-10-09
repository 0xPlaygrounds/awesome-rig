// rig_agent.rs

use anyhow::{Context, Result};
use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::VectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::agent::Agent;
use rig::completion::Prompt;
use std::path::Path;
use std::fs;
use std::sync::Arc;

pub struct RigAgent {
    agent: Arc<Agent<openai::CompletionModel>>,
}

impl RigAgent {
    pub async fn new() -> Result<Self> {
        // Initialize OpenAI client
        let openai_client = openai::Client::from_env();
        let embedding_model = openai_client.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

        // Create vector store
        let mut vector_store = InMemoryVectorStore::default();

        // Get the current directory and construct paths to markdown files
        let current_dir = std::env::current_dir()?;
        let documents_dir = current_dir.join("documents");

        let md1_path = documents_dir.join("Rig_guide.md");
        let md2_path = documents_dir.join("Rig_faq.md");
        let md3_path = documents_dir.join("Rig_examples.md");

        // Load markdown documents
        let md1_content = Self::load_md_content(&md1_path)?;
        let md2_content = Self::load_md_content(&md2_path)?;
        let md3_content = Self::load_md_content(&md3_path)?;

        // Create embeddings and add to vector store
        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .simple_document("Rig_guide", &md1_content)
            .simple_document("Rig_faq", &md2_content)
            .simple_document("Rig_examples", &md3_content)
            .build()
            .await?;

        vector_store.add_documents(embeddings).await?;

        // Create index
        let index = vector_store.index(embedding_model);

        // Create Agent
        let agent = Arc::new(openai_client.agent(openai::GPT_4O)
            .preamble("You are an advanced AI assistant powered by Rig, a Rust library for building LLM applications. Your primary function is to provide accurate, helpful, and context-aware responses by leveraging both your general knowledge and specific information retrieved from a curated knowledge base.

                    Key responsibilities and behaviors:
                    1. Information Retrieval: You have access to a vast knowledge base. When answering questions, always consider the context provided by the retrieved information.
                    2. Clarity and Conciseness: Provide clear and concise answers. Ensure responses are short and concise. Use bullet points or numbered lists for complex information when appropriate.
                    3. Technical Proficiency: You have deep knowledge about Rig and its capabilities. When discussing Rig or answering related questions, provide detailed and technically accurate information.
                    4. Code Examples: When appropriate, provide Rust code examples to illustrate concepts, especially when discussing Rig's functionalities. Always format code examples for proper rendering in Discord by wrapping them in triple backticks and specifying the language as 'rust'. For example:
                        ```rust
                        let example_code = \"This is how you format Rust code for Discord\";
                        println!(\"{}\", example_code);
                        ```
                    5. Keep your responses short and concise. If the user needs more information, they can ask follow-up questions.
                    ")
            .dynamic_context(2, index)
            .build());

        Ok(Self { agent })
    }

    fn load_md_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
        fs::read_to_string(file_path.as_ref())
            .with_context(|| format!("Failed to read markdown file: {:?}", file_path.as_ref()))
    }

    pub async fn process_message(&self, message: &str) -> Result<String> {
        self.agent.prompt(message).await.map_err(anyhow::Error::from)
    }
}