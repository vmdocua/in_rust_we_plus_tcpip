mod echo_server;

use log4rs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yaml;
use echo_server::echo_server_run;

// App Config
#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";


#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    echo: EchoConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct EchoConfig {
    #[serde(default = "default_true")]
    enabled: bool,
    host: String,
    port: u16,
}

fn default_true() -> bool {
    true
}

fn init_logger() {
    // Load logging configuration from log4rs.yaml file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

fn load_config(file_path: &str) -> AppConfig {
    // Open and read the configuration file
    let file = std::fs::File::open(file_path).expect("Failed to open config file");
    let reader = std::io::BufReader::new(file);

    // Parse YAML contents into ServerConfig struct
    return serde_yaml::from_reader(reader).expect("Failed to parse config")
}

fn main() {
    println!("Hi, in_rust_we_plus TCP/IP!");

    println!("Version: {} ({})", env!("CARGO_PKG_VERSION"), BUILD_TYPE);

    init_logger();
    debug!("Logger initialized");

    let cfg: AppConfig = load_config("config.yaml");
    debug!("Config loaded: {:?}", cfg);
    if cfg.echo.enabled {
        echo_server_run(&cfg.echo.host, cfg.echo.port)
    } else {
        info!("Echo server disabled");
    }

    info!("Done.");
}
