use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::VectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::completion::Prompt;
use std::io::{self, Write};
use std::path::Path;
use anyhow::{Result, Context};
use pdf_extract::extract_text;

fn load_pdf_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
    extract_text(file_path.as_ref())
        .with_context(|| format!("Failed to extract text from PDF: {:?}", file_path.as_ref()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let openai_client = openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

    let mut vector_store = InMemoryVectorStore::default();

    // Get the current directory and construct paths to PDF files
    let current_dir = std::env::current_dir()?;
    let documents_dir = current_dir.join("documents");

    let pdf1_path = documents_dir.join("Moores_Law_for_Everything.pdf");
    let pdf2_path = documents_dir.join("The_Last_Question.pdf");

    // Load PDF documents
    let pdf1_content = load_pdf_content(&pdf1_path)?;
    let pdf2_content = load_pdf_content(&pdf2_path)?;

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("Moores_Law_for_Everything", &pdf1_content)
        .simple_document("The_Last_Question", &pdf2_content)
        .build()
        .await?;

    vector_store.add_documents(embeddings).await?;

    let rag_agent = openai_client.context_rag_agent("gpt-3.5-turbo")
        .preamble("You are a helpful assistant that answers questions based on the given context from PDF documents.")
        .dynamic_context(2, vector_store.index(embedding_model))
        .build();

    println!("RAG System ready. Type 'exit' to quit.");

    loop {
        print!("Enter your question: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let response = rag_agent.prompt(input).await?;
        println!("Response: {}", response);
    }

    Ok(())
}