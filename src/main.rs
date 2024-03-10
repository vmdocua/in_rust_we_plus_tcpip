use log::{debug, info};
use in_rust_we_plus_tcpip::{init_logger, load_config, AppConfig};
use in_rust_we_plus_tcpip::echo_server::echo_server_run;


// App Config
#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";


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
