use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Other(String),
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Entity {
    entity_type: EntityType,
    name: String,
    confidence: f32,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct ExtractedEntities {
    entities: Vec<Entity>,
    total_count: usize,
    extraction_time: String, // ISO 8601 formatted string
}

fn pretty_print_entities(extracted: &ExtractedEntities) {
    println!("Extracted Entities:");
    println!("Total Count: {}", extracted.total_count);
    println!("Extraction Time: {}", extracted.extraction_time);
    println!("Entities:");
    for entity in &extracted.entities {
        println!(
            "  - Type: {:?}, Name: {}, Confidence: {:.2}",
            entity.entity_type, entity.name, entity.confidence
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Create the extractor
    let extractor = openai_client
        .extractor::<ExtractedEntities>("gpt-4")
        .preamble("You are an AI assistant specialized in extracting named entities from text. \
                   Your task is to identify and categorize entities such as persons, organizations, \
                   locations, and dates. Provide a confidence score for each entity identified.")
        .build();

    // Sample text for entity extraction
    let sample_text = "On July 20, 1969, Neil Armstrong and Buzz Aldrin, astronauts from NASA, \
                       became the first humans to land on the Moon as part of the Apollo 11 mission. \
                       The historic event was broadcast live by CBS News, anchored by Walter Cronkite \
                       from New York City.";

    println!("Extracting entities from the following text:\n{}\n", sample_text);

    // Extract entities
    match extractor.extract(sample_text).await {
        Ok(extracted_entities) => {
            pretty_print_entities(&extracted_entities);
        }
        Err(e) => eprintln!("Error extracting entities: {}", e),
    }

    Ok(())
}