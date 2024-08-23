use rig::providers::openai;
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct PersonData {
    name: String,
    age: u8,
    email: String,
    occupation: String,
    favorite_color: String,
}

fn pretty_print_person(person: &PersonData) {
    println!("Generated Person Data:");
    println!("  Name: {}", person.name);
    println!("  Age: {}", person.age);
    println!("  Email: {}", person.email);
    println!("  Occupation: {}", person.occupation);
    println!("  Favorite Color: {}", person.favorite_color);
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Create the data generator
    let data_generator = openai_client
        .model("gpt-4")
        .build();

    // Define the schema and instructions
    let schema_and_instructions = r#"
    Generate synthetic personal data based on the following schema:
    {
        "name": "String (full name)",
        "age": "Integer (18-80)",
        "email": "String (valid email format)",
        "occupation": "String",
        "favorite_color": "String"
    }

    Instructions:
    1. Generate realistic and diverse data.
    2. Ensure email addresses are in a valid format but fictional.
    3. Vary the occupations and favorite colors.
    4. Provide the data in JSON format.

    Generate 5 unique entries.
    "#;

    // Generate synthetic data
    let generated_data = data_generator.prompt(schema_and_instructions).await?;

    // Parse and print the generated data
    let people: Vec<PersonData> = serde_json::from_str(&generated_data)?;
    
    for person in people {
        pretty_print_person(&person);
    }

    Ok(())
}