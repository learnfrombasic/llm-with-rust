use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub gemini_api_key: String,
}

impl Config {
    pub fn new() -> Config {
        let gemini_api_key = env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY environment variable not set. Please create a .env file with GEMINI_API_KEY=your_api_key_here");
            
        Self { gemini_api_key }
    }
}
