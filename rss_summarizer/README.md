# Hacker News RSS Feed Summarizer using [Rig](https://github.com/0xPlaygrounds/rig)

This project demonstrates how to leverage [Rig](https://github.com/0xPlaygrounds/rig), a powerful Rust library for building LLM-powered applications, to create an AI agent that summarizes RSS feeds from Hacker News. The summarizer fetches the latest news articles every hour, processes them using an AI model, and outputs concise summaries along with relevance scores. This project is a great starting point for anyone interested in AI-driven content summarization.

### What is an RSS Feed?

RSS (Really Simple Syndication) is a type of web feed that allows users and applications to receive regular updates from websites. For example, an RSS feed from a news website might provide the latest headlines, summaries, and links to full articles. This project focuses on summarizing the RSS feed from Hacker News, a popular site for tech and startup news.

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Clone this repository or create a new Rust project:
   ```
   cargo new hn-rss-summarizer
   cd hn-rss-summarizer
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig = "0.1.0"
   serde = { version = "1.0", features = ["derive"] }
   chrono = { version = "0.4", features = ["serde"] }
   rss = "2.0"
   tokio = { version = "1.0", features = ["full"] }
   reqwest = { version = "0.11", features = ["json"] }
   regex = "1"
   schemars = "0.8"
   ```

3. Set your OpenAI API key as an environment variable:
   ```bash
   export OPENAI_API_KEY=your_api_key_here
   ```

### Code Overview

The main components of this example are:

1. **Fetching the RSS Feed**:
   This function fetches the RSS feed from Hacker News using the `reqwest` crate and parses it into a `Channel` object using the `rss` crate.

   ```rust
   async fn fetch_rss_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
       let response = reqwest::get(url).await?.text().await?;
       let channel = response.parse::<Channel>()?;
       Ok(channel)
   }
   ```

2. **Sanitizing and Summarizing Feed Items**:
   We sanitize the RSS item descriptions by removing HTML tags and other unwanted characters using the `regex` crate, then summarize the feed using an AI model with Rig.

   ```rust
   let re = Regex::new(r"<[^>]*>").unwrap();
   let clean_description = re.replace_all(&description, "").to_string();
   ```

3. **AI-Based Summarization**:
   An AI extractor is set up using Rig to analyze the RSS feed items and generate concise summaries with relevance scores.

   ```rust
   let extractor = openai_client
       .extractor::<RssSummary>("gpt-4")
       .preamble("You are an AI assistant specialized in summarizing RSS feeds...")
       .build();
   ```

4. **Periodic Fetching and Summarization**:
   The main function sets up a loop to fetch and summarize the RSS feed every hour using `tokio::time::interval`.

   ```rust
   let mut interval = time::interval(Duration::from_secs(3600));
   loop {
       interval.tick().await;
       match fetch_rss_feed(rss_url).await {
           // Handling fetch and summarization logic
       }
   }
   ```

### Running the Example

1. Ensure all dependencies are listed in your `Cargo.toml`.
2. Run the example using:
   ```bash
   cargo run
   ```

### Understanding the Code

Here’s a breakdown of the key parts:

- **RSS Fetching**: We use `reqwest` to fetch the RSS feed and `rss` crate to parse it.
- **Sanitization**: HTML tags and unnecessary characters are removed to clean the RSS content.
- **Summarization**: Rig, coupled with OpenAI's GPT-4 model, is employed to generate summaries.
- **Periodic Execution**: Using `tokio`, the fetch-summarize loop runs every hour, automatically fetching new content and generating fresh summaries.

### Customization

Feel free to customize the `main` function's interval timing or modify the summarization prompt to adjust the level of detail or style of the summaries. You can also change the RSS feed URL to summarize different content.

### Troubleshooting

If you encounter any issues:

- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.
- If you receive errors related to the RSS feed parsing, ensure the feed URL is correct and accessible.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).