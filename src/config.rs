use serde::Deserialize;
use std::env;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub memory_size_limit: usize,
}

impl Settings {
    pub fn new() -> Self {
        let current_dir = env::current_dir()
            .expect("Failed to get current directory");
        let config_path = current_dir.join("config").join("config.json");
        let file = File::open(&config_path)
            .expect("config.json file not found");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .expect("Error parsing JSON")
    }
}
