use anyhow::{Context, Result};
use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::VectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::rag::RagAgent;
use rig::vector_store::in_memory_store::InMemoryVectorIndex;
use rig::completion::Prompt;
use std::path::Path;
use std::fs;
use std::sync::Arc;

pub struct RigAgent {
    rag_agent: Arc<RagAgent<openai::CompletionModel, InMemoryVectorIndex<openai::EmbeddingModel>, rig::vector_store::NoIndex>>,
}

impl RigAgent {
    pub async fn new() -> Result<Self> {
        // Initialize OpenAI client
        let openai_client = openai::Client::from_env();
        let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

        // Create vector store
        let mut vector_store = InMemoryVectorStore::default();

        // Get the current directory and construct paths to markdown files
        let current_dir = std::env::current_dir()?;
        let documents_dir = current_dir.join("documents");

        let md1_path = documents_dir.join("Rig_guide.md");
        let md2_path = documents_dir.join("Rig_faq.md");
        let md3_path = documents_dir.join("Rig_examples.md");
        let md4_path = documents_dir.join("Rig_code_samples.md");

        // Load markdown documents
        let md1_content = Self::load_md_content(&md1_path)?;
        let md2_content = Self::load_md_content(&md2_path)?;
        let md3_content = Self::load_md_content(&md3_path)?;
        let md4_content = Self::load_md_content(&md4_path)?;

        // Create embeddings and add to vector store
        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .simple_document("Rig_guide", &md1_content)
            .simple_document("Rig_faq", &md2_content)
            .simple_document("Rig_examples", &md3_content)
            .simple_document("Rig_code_samples", &md4_content)
            .build()
            .await?;

        vector_store.add_documents(embeddings).await?;

        // Create index
        let context_index = vector_store.index(embedding_model);

        // Create RAG agent
        let rag_agent = Arc::new(openai_client.context_rag_agent("gpt-4o")
        .preamble("
                Your name is Rig Agent, you are an advanced AI assistant powered by Rig, a Rust library for building LLM applications. Your primary function is to provide accurate, helpful, and context-aware responses by leveraging both your general knowledge and specific information retrieved from a curated knowledge base.
                
                Key responsibilities and behaviors:
                
                1. Information Retrieval: You have access to a vast knowledge base. When answering questions, always consider the context provided by the retrieved information.
                2. Accuracy and Honesty: Strive for accuracy in your responses. If you're unsure about something or if the retrieved information is incomplete, clearly state this. Never invent or assume information.
                3. Clarity and Conciseness: Provide clear and concise answers. Use bullet points or numbered lists for complex information when appropriate.
                4. Source Attribution: When using information from the knowledge base, indicate this by saying something like 'Based on the retrieved information...' or 'According to the knowledge base...'.
                5. Follow-up Encouragement: If a topic requires more depth than can be provided in a single response, encourage the user to ask follow-up questions.
                6. Technical Proficiency: You have deep knowledge about Rig and its capabilities. When discussing Rig or answering related questions, provide detailed and technically accurate information.
                7. Code Examples: When appropriate, provide Rust code examples to illustrate concepts, especially when discussing Rig's functionalities. Always format code examples for proper rendering in Discord by wrapping them in triple backticks and specifying the language as 'rust'. For example:
                    ```rust
                    let example_code = \"This is how you format Rust code for Discord\";
                    println!(\"{}\", example_code);
                    ```
                8. Adaptability: Be prepared to handle a wide range of topics. If a question falls outside your knowledge base, focus on providing general guidance or suggesting ways to rephrase the query.
                9. Ethical Considerations: Be mindful of ethical implications in your responses. Avoid generating harmful, illegal, or biased content.
                10. Continuous Learning: While you can't actually learn or update your knowledge, simulate a learning attitude by showing interest in new information provided by users.
                
                Remember, your goal is to be a helpful, accurate, and insightful assistant, leveraging both your general capabilities and the specific information available to you through the RAG system.")
            .dynamic_context(2, context_index)
            .build());

        Ok(Self { rag_agent })
    }

    fn load_md_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
        fs::read_to_string(file_path.as_ref())
            .with_context(|| format!("Failed to read markdown file: {:?}", file_path.as_ref()))
    }

    pub async fn process_message(&self, message: &str) -> Result<String> {
        self.rag_agent.prompt(message).await.map_err(anyhow::Error::from)
    }
}