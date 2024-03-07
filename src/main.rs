mod echo_server;

use echo_server::echo_server_run;

fn main() {
    println!("Hi, in_rust_we_plus TCP/IP!");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));

    echo_server_run("127.0.0.1", 3535);

    println!("Done.");
}
