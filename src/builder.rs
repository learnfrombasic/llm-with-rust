use std::env;

use rig::{agent::AgentBuilder, providers::{gemini, openai, groq}};
// TODO: add ollama-rs

pub struct AgentConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub preamble: String,
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


pub fn agent(config: AgentConfig) -> AgentBuilder {
    match config.provider.as_str() {
        "gemini" => {
            let client = gemini::Client::new(&config.api_key)
                .agent(&config.model)
                .preamble(&config.preamble)
                .temperature(config.temperature)
                .build();
            client
        }, 
        "openai" => {
            let client = openai::Client::new(&config.api_key)
                .agent(&config.model)
                .preamble(&config.preamble)
                .temperature(config.temperature)
                .build();
            client
        },
        "groq" => {
            let client = groq::Client::new(&config.api_key)
                .agent(&config.model)
                .preamble(&config.preamble)
                .temperature(config.temperature)
                .build();
            client
        },
        "ollama" => {
            let client = ollama::Client::new(&config.api_key)
                .agent(&config.model)
                .preamble(&config.preamble)
                .temperature(config.temperature)
                .build();
            client
        },
        _ => panic!("Unsupported provider"),
    }
}