use agent_state_machine::ChatAgentStateMachine;
use rig::providers::openai::{self, GPT_4};
use rig::completion::{ToolDefinition};
use rig::tool::Tool;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct SearchArgs {
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArxivResult {
    title: String,
    summary: String,
    link: String,
}

#[derive(Debug, Deserialize)]
struct ArxivApiResponse {
    #[serde(rename = "feed")]
    feed: Feed,
}

#[derive(Debug, Deserialize)]
struct Feed {
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
    summary: String,
    id: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Search error: {0}")]
struct SearchError(String);

#[derive(Clone)]
struct ArxivSearch {
    client: reqwest::Client,
}

impl ArxivSearch {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn search(&self, query: &str) -> Result<Vec<ArxivResult>, SearchError> {
        let url = format!(
            "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=5",
            urlencoding::encode(query)
        );

        let response = self.client.get(&url).send().await.map_err(|e| SearchError(e.to_string()))?;
        let response_text = response.text().await.map_err(|e| SearchError(e.to_string()))?;
        let response_json: ArxivApiResponse = serde_xml_rs::from_str(&response_text).map_err(|e| SearchError(e.to_string()))?;

        let results = response_json
            .feed
            .entries
            .into_iter()
            .map(|entry| ArxivResult {
                title: entry.title,
                summary: entry.summary,
                link: entry.id,
            })
            .collect();

        Ok(results)
    }
}

impl Tool for ArxivSearch {
    const NAME: &'static str = "arxiv_search";
    type Error = SearchError;
    type Args = SearchArgs;
    type Output = Vec<ArxivResult>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search for academic papers on arXiv.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to look for papers on arXiv"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        self.search(&args.query).await
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ResearchState {
    Ready,
    Searching,
    Summarizing,
    Complete,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = openai::Client::from_env();

    // Create ArxivSearch tool
    let arxiv_search_tool = ArxivSearch::new();

    // Create a basic chat agent with the ArxivSearch tool
    let agent = openai_client
        .agent(GPT_4)
        .preamble("You are a helpful assistant with academic search capabilities using arXiv. When providing search results, summarize the main points and present a concise summary of the key information from the top few results.")
        .tool(arxiv_search_tool.clone())
        .build();

    // Create a state machine for managing the agent
    let mut state_machine = ChatAgentStateMachine::new(agent);

    // Subscribe to state changes
    let mut state_rx = state_machine.subscribe_to_state_changes();
    tokio::spawn(async move {
        while let Ok(state) = state_rx.recv().await {
            println!("📍 State: {}", state);
        }
    });

    // Process a query using the state machine
    let response = state_machine.process_message("Search for the latest research on quantum computing").await?;
    println!("Response: {}", response);

    // Small delay to make the interaction feel more natural
    tokio::time::sleep(Duration::from_millis(500)).await;

    Ok(())
}
