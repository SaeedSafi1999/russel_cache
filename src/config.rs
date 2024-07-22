use serde::Deserialize;
use std::env;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
}

// impl Settings {
//     pub fn new() -> Self {
//         let current_dir_os_string = std::env::current_dir().unwrap().into_os_string();
//         let current_dir: &str = current_dir_os_string.to_str().expect("invaild char in config path");        
//         let config_path =format!("{}\\config\\config.json",current_dir);
//         let file = File::open(config_path).expect("config.json file not found");
//         let reader = BufReader::new(file);
//         serde_json::from_reader(reader).expect("Error parsing JSON")
//     }
// }

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
