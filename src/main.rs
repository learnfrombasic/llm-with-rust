mod builder;
use rig::{completion::Prompt, providers::gemini};
use std::error::Error;

// use crate::config::Config;
use crate::builder::{AgentConfig, completion_builder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let config = Config::new();
    let agent = gemini::Client::new(&config.gemini_api_key)
        .agent("gemini-2.0-flash")
        .preamble("You are comedian AI with a mission to make people laugh.")
        .temperature(0.8)
        .build();

    let query: String = "Tell me a joke".to_string();
    let response = agent.prompt(query).await.expect("Failed to get response");
    println!("Response: {:?}", response);
    Ok(())
}
