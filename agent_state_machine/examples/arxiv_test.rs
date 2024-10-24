use reqwest;
use serde::Deserialize;
use serde_xml_rs;

#[derive(Debug, Deserialize)]
struct ArxivApiResponse {
    #[serde(rename = "feed")]
    feed: Option<Feed>,
}

#[derive(Debug, Deserialize)]
struct Feed {
    #[serde(rename = "entry")]
    entries: Option<Vec<Entry>>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
    summary: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "quantum computing";
    let url = format!(
        "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=5",
        urlencoding::encode(query)
    );

    let response = reqwest::get(&url).await?;
    let response_text = response.text().await?;

    // Print the response text for debugging purposes
    println!("Response text: {}", response_text);
    
    let response_json: ArxivApiResponse = serde_xml_rs::from_str(&response_text)?;

    if let Some(feed) = response_json.feed {
        if let Some(entries) = feed.entries {
            for entry in entries {
                println!("Title: {}\nSummary: {}\nLink: {}\n", entry.title, entry.summary, entry.id);
            }
        } else {
            println!("No entries found in the feed.");
        }
    } else {
        println!("No feed found in the response.");
    }

    Ok(())
}