# Building a RAG Agent over PDF files using Rig

## Overview

This project demonstrates a Retrieval-Augmented Generation (RAG) system built with Rig, a Rust library for developing LLM-powered applications. The system processes PDF documents, creates embeddings, and uses OpenAI's GPT-3.5-turbo model to answer questions based on the content of these documents.

In this example, we use two PDF documents:
1. "Moore's Law for Everything" by Sam Altman
2. "The Last Question" by Isaac Asimov

## Features

- PDF text extraction
- Document embedding using OpenAI's text-embedding-ada-002 model
- In-memory vector store for quick retrieval
- Dynamic context generation for each query
- Interactive Q&A interface

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, sign up at [OpenAI's website](https://openai.com).

## Setup

1. Clone this repository:
   ```
   git clone this repo
   cd pdf-rag-system
   ```

2. Set your OpenAI API key as an environment variable:
   ```
   export OPENAI_API_KEY=your_api_key_here
   ```

3. Ensure you have the following PDF documents in a `documents` folder in your project root:
   - `Moores_Law_for_Everything.pdf`
   - `The_Last_Question.pdf`

## Running the Application

1. Build and run the application:
   ```
   cargo run
   ```

2. Once the system is ready, you'll see the message: "RAG System ready. Type 'exit' to quit."

3. Enter your questions at the prompt. The system will provide answers based on the content of the PDF documents.

4. To exit the application, type 'exit' at the prompt.

## Example Usage

```
RAG System ready. Type 'exit' to quit.
Enter your question: tell me the premise of what sam altman is talking about
Response: Sam Altman discusses the coming technological revolution driven by powerful artificial intelligence (AI) systems that can think, learn, and perform tasks currently done by people. He highlights how this AI revolution will lead to the creation of phenomenal wealth but also emphasizes the need for policy changes to distribute this wealth and ensure inclusivity in society. Altman proposes the idea of embracing AI advancements, transitioning taxation from labor to capital (such as companies and land), and distributing wealth equitably through the American Equity Fund. This plan aims to improve the standard of living for everyone by leveraging technology and fair economic policies in a rapidly changing future.
Enter your question: 
```

## How It Works

1. **PDF Processing**: The system extracts text from the specified PDF documents.
2. **Embedding Creation**: It generates embeddings for the extracted text using OpenAI's embedding model.
3. **Vector Store**: The embeddings are stored in an in-memory vector store for quick retrieval.
4. **Query Processing**: When a user enters a question, the system:
   a. Generates an embedding for the question.
   b. Retrieves the most relevant context from the vector store.
   c. Sends the question and context to the GPT-3.5-turbo model.
   d. Returns the model's response to the user.

## Customization

- To use different PDF documents, place them in the `documents` folder and update the file paths in the `main` function.
- You can adjust the number of relevant documents retrieved for each query by changing the `dynamic_context` parameter.
- To use a different OpenAI model, modify the model name in the `context_rag_agent` function call.

## Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Verify that the PDF documents are in the correct location and are readable.
- Check that all dependencies are properly installed by running `cargo build`.

## Dependencies

This project uses the following main dependencies:
- `rig-core`: For building LLM-powered applications
- `pdf-extract`: For extracting text from PDF files
- `tokio`: For asynchronous runtime
- `anyhow`: For error handling

For a complete list of dependencies, refer to the `Cargo.toml` file.


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.