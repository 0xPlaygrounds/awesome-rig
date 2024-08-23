use anyhow::Result;
use rig::completion::{Chat, Message};
use rig::model::ModelBuilder;
use rig::providers::cohere::{self, Client};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Cohere client
    let cohere = Client::new(&std::env::var("COHERE_API_KEY")?);

    // Create a Cohere model
    let model = ModelBuilder::new(cohere.completion_model(cohere::COMMAND))
        .temperature(0.7)
        .build();

    // Define our context
    let context = "
    The Rust programming language was initially designed and developed by Mozilla employee Graydon Hoare as a personal project. 
    Mozilla began sponsoring the project in 2009 and announced it in 2010. 
    Rust 1.0, the first stable release, was released on May 15, 2015.
    Rust is syntactically similar to C++, but provides memory safety without using garbage collection.
    Rust has been voted the 'most loved programming language' in the Stack Overflow Developer Survey every year since 2016.
    ";

    // Create our chat history with the context
    let mut chat_history = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant that answers questions based on the given context.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: format!("Here's some context for you to use: {}", context),
        },
        Message {
            role: "assistant".to_string(),
            content: "Thank you for providing the context about Rust. I'm ready to answer any questions you may have about it.".to_string(),
        },
    ];

    // Main interaction loop
    loop {
        println!("Ask a question about Rust (or type 'exit' to quit):");
        let mut question = String::new();
        std::io::stdin().read_line(&mut question)?;
        question = question.trim().to_string();

        if question.to_lowercase() == "exit" {
            break;
        }

        chat_history.push(Message {
            role: "user".to_string(),
            content: question,
        });

        // Get the model's response
        let response = model.chat(&chat_history.last().unwrap().content, chat_history.clone()).await?;

        println!("Answer: {}", response);

        chat_history.push(Message {
            role: "assistant".to_string(),
            content: response,
        });
    }

    Ok(())
}