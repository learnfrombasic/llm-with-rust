use std::env;

use rig::completion::CompletionModel;
use rig::providers::{gemini, groq, ollama, openai};
pub struct AgentConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub preamble: String, // System prompt
    pub temperature: f64,
}

impl AgentConfig {
    pub fn new() -> Self {
        Self {
            provider: env::var("PROVIDER").expect("PROVIDER environment variable not set"),
            api_key: env::var("API_KEY").expect("API_KEY environment variable not set"),
            model: env::var("MODEL").expect("MODEL environment variable not set"),
            preamble: env::var("PREAMBLE").expect("PREAMBLE environment variable not set"),
            temperature: env::var("TEMPERATURE")
                .expect("TEMPERATURE environment variable not set")
                .parse()
                .expect("TEMPERATURE environment variable must be a number"),
        }
    }
}

pub fn completion_builder(config: AgentConfig) -> Result<CompletionModel, String> {
    match config.provider.as_str() {
        "gemini" => gemini::Client::new(&config.api_key)
            .agent(&config.model)
            .preamble(&config.preamble)
            .temperature(config.temperature)
            .build(),
        "openai" => openai::Client::new(&config.api_key)
            .agent(&config.model)
            .preamble(&config.preamble)
            .temperature(config.temperature)
            .build(),
        "groq" => groq::Client::new(&config.api_key)
            .agent(&config.model)
            .preamble(&config.preamble)
            .temperature(config.temperature)
            .build(),
        "ollama" => ollama::Client::new()
            .agent(&config.model)
            .preamble(&config.preamble)
            .temperature(config.temperature)
            .build(),
        _ => Err("Unsupported provider".to_string()),
    }
}
