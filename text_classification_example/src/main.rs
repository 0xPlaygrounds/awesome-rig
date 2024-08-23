use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Category {
    Technology,
    Science,
    Politics,
    Sports,
    Entertainment,
    Other(String),
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct ClassificationResult {
    category: Category,
    confidence: f32,
    summary: String,
}

fn pretty_print_result(text: &str, result: &ClassificationResult) {
    println!("Text: \"{}\"", text);
    println!("Classification Result:");
    println!("  Category: {:?}", result.category);
    println!("  Confidence: {:.2}%", result.confidence * 100.0);
    println!("  Summary: {}", result.summary);
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Create the classifier
    let classifier = openai_client
        .extractor::<ClassificationResult>("gpt-4")
        .preamble(
            "You are an AI assistant specialized in classifying text into predefined categories. \
            The categories are: Technology, Science, Politics, Sports, and Entertainment. \
            If the text doesn't fit into these categories, use the Other category and specify a suitable label. \
            Provide a confidence score and a brief summary for each classification."
        )
        .build();

    // Sample texts for classification
    let sample_texts = vec![
        "Apple announced its new M2 chip, promising significant performance improvements for MacBooks.",
        "Scientists have discovered a new exoplanet that could potentially harbor life.",
        "The upcoming election is expected to be one of the most closely contested in recent history.",
        "The underdog team pulled off a stunning victory in the championship final.",
        "The latest blockbuster movie broke box office records in its opening weekend.",
        "The annual flower show attracted gardening enthusiasts from across the country.",
    ];

    // Classify each sample text
    for text in sample_texts {
        match classifier.extract(text).await {
            Ok(result) => pretty_print_result(text, &result),
            Err(e) => eprintln!("Error classifying text: {}", e),
        }
    }

    Ok(())
}