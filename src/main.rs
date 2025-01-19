use clap::Parser;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser)]
#[command(name = "mim")]
#[command(about = "Send message to Anthropic API")]
struct Cli {
    /// The message to send
    #[arg(required = true, num_args = 1..)]
    message: Vec<String>,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct Content {
    text: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();
    
    // Join the message words into a single string
    let message_text = cli.message.join(" ");
    
    // Get API key from environment variable
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set");

    // Create the request payload
    let request_payload = AnthropicRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![
            Message {
                role: "user".to_string(),
                content: message_text,
            }
        ],
    };

    // Create HTTP client
    let client = reqwest::Client::new();

    // Send request to Anthropic API
    let response = client.post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&request_payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the response
        let anthropic_response: AnthropicResponse = response.json().await?;
        
        // Print the response text
        if let Some(content) = anthropic_response.content.first() {
            if let Some(text) = &content.text {
                println!("Response: {}", text);
            }
        }
    } else {
        // Print error details if the request failed
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
    }

    Ok(())
}