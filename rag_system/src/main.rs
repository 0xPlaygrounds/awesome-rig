use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::VectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::cli_chatbot::cli_chatbot;  // Import the cli_chatbot function
use std::path::Path;
use anyhow::{Result, Context};
use pdf_extract::extract_text;

fn load_pdf_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
    extract_text(file_path.as_ref())
        .with_context(|| format!("Failed to extract text from PDF: {:?}", file_path.as_ref()))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

    // Create vector store
    let mut vector_store = InMemoryVectorStore::default();

    // Get the current directory and construct paths to PDF files
    let current_dir = std::env::current_dir()?;
    let documents_dir = current_dir.join("documents");

    let pdf1_path = documents_dir.join("Moores_Law_for_Everything.pdf");
    let pdf2_path = documents_dir.join("The_Last_Question.pdf");

    // Load PDF documents
    let pdf1_content = load_pdf_content(&pdf1_path)?;
    let pdf2_content = load_pdf_content(&pdf2_path)?;

    // Create embeddings and add to vector store
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("Moores_Law_for_Everything", &pdf1_content)
        .simple_document("The_Last_Question", &pdf2_content)
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    // Create RAG agent
    let rag_agent = openai_client.context_rag_agent("gpt-3.5-turbo")
        .preamble("You are a helpful assistant that answers questions based on the given context from PDF documents.")
        .dynamic_context(2, vector_store.index(embedding_model))
        .build();

    // Use the cli_chatbot function to create the CLI interface
    cli_chatbot(rag_agent).await?;

    Ok(())
}