mod echo_server;

use log4rs;
use log::{debug};
use echo_server::echo_server_run;

pub fn init_logger() {
    // Load logging configuration from log4rs.yaml file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

fn main() {
    println!("Hi, in_rust_we_plus TCP/IP!");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));

    init_logger();
    debug!("Logger initialized");
    echo_server_run("127.0.0.1", 3535);

    println!("Done.");
}
