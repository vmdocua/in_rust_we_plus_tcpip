pub mod echo_server;

use log4rs;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub echo: EchoConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

pub fn default_true() -> bool {
    true
}

pub fn init_logger() {
    // Load logging configuration from log4rs.yaml file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

pub fn load_config(file_path: &str) -> AppConfig {
    // Open and read the configuration file
    let file = std::fs::File::open(file_path).expect("Failed to open config file");
    let reader = std::io::BufReader::new(file);

    // Parse YAML contents into ServerConfig struct
    return serde_yaml::from_reader(reader).expect("Failed to parse config")
}
