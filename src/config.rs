use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub threads: usize,
    pub timeout: u64,
    pub input_file: String,
    pub output_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: 1500,
            timeout: 10,
            input_file: "cidrs.txt".to_string(),
            output_file: "port_open.txt".to_string(),
        }
    }
}

impl Config {
    pub fn load(config_path: String) -> Result<Self, Box<dyn Error>> {
        if !std::path::Path::new(&config_path).exists() {
            confy::store_path(config_path.clone(), Config::default()).unwrap();

            return Err(format!("File '{}' not found!", config_path).into());
        }

        match confy::load_path(config_path.clone()) {
            Ok(cfg) => return Ok(cfg),
            Err(_) => {
                return Err(format!("Error in '{}'", config_path).into());
            }
        };
    }
}
