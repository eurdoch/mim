use clap::Parser;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser)]
#[command(name = "mim")]
#[command(about = "Generate and execute bash command using Anthropic API")]
struct Cli {
    /// The user request for a bash command
    #[arg(required = true, num_args = 1..)]
    request: Vec<String>,

    /// Automatically execute the command without asking
    #[arg(short, long)]
    yes: bool,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
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
    
    // Join the request words into a single string
    let user_request = cli.request.join(" ");
    
    // Get API key from environment variable
    let api_key = env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set");

    // Create the request payload with system and user messages
    let request_payload = AnthropicRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1024,
        system: "You are an expert bash command generator. When given a user request, respond with ONLY a single, concise bash command that precisely accomplishes the task. Do not include any explanation, commentary, or additional text - just the exact bash command needed.".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: user_request,
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
    let bash_command = if response.status().is_success() {
        // Parse the response
        let anthropic_response: AnthropicResponse = response.json().await?;
        
        // Extract the command text
        if let Some(content) = anthropic_response.content.first() {
            content.text.clone().unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        // Print error details if the request failed
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
        String::new()
    };

    // Trim the command and print
    let bash_command = bash_command.trim();
    println!("Generated Bash Command: {}", bash_command);

    // Determine if we should execute
    let should_execute = if cli.yes {
        true
    } else {
        // Prompt user for confirmation
        print!("Do you want to execute this command? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        input.trim().eq_ignore_ascii_case("y")
    };

    // Execute if confirmed
    if should_execute {
        // Execute the command
        let output = Command::new("bash")
            .arg("-c")
            .arg(bash_command)
            .output()?;

        // Print stdout
        if !output.stdout.is_empty() {
            println!("Command Output:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // Print stderr if there are any errors
        if !output.stderr.is_empty() {
            eprintln!("Command Errors:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        // Print exit status
        println!("Command exited with status: {}", output.status);
    } else {
        println!("Command execution cancelled.");
    }

    Ok(())
}