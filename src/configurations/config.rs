use serde::Deserialize;
use std::fs::File;
use std::io::Read;


#[derive(Debug, Deserialize)]
pub struct Config {
    port: u16,
    max_ram_limit: usize, // Size in bytes
}

pub fn load_config(file_path: &str) -> Result<Config, serde_json::Error> {
    let mut file = File::open(file_path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");
    serde_json::from_str(&contents)
}