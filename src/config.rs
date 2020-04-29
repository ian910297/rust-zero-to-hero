use json::JsonValue;
use serde::{Deserialize, Serialize};

use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    username: String,
    password: String,
    repo: String,
    bello: Option<String>
}

impl Config {
    // Copy from the following link
    // Beginner friendly example with json file https://github.com/serde-rs/serde/issues/1195
    pub fn new(path: String) -> Config {
        let json_file_path = Path::new("env.json");
        let json_file_str = read_to_string(json_file_path).expect("file not found");

        // use instead of from_reader
        let config: Config = serde_json::from_str(&json_file_str).expect("error while reading json");
        println!("{:?}", config);
        config
    }
}
