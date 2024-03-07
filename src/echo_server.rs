use log::{info, error};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn echo_connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    info!("Connection closed");
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                info!("<<: {}", message);
                if message.trim() == "shutdown" {
                    info!("Shutdown signal received. Closing connection.");
                    // TODO: Send a shutdown signal to the server
                    return;
                }
                if let Err(_) = stream.write_all(&buffer[0..n]) {
                    error!("Error writing to stream");
                    break;
                } else {
                    info!(">>: {}", message);
                }
            }
            Err(_) => {
                error!("Error reading from stream");
                break;
            }
        }
    }
}

pub fn echo_server_run(host: &str, port: u16) {
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address)
        .expect(&format!("Failed to bind to address: {}", address));
    info!("Echo server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_addr = stream.peer_addr().expect("Failed to get client address");
                let server_addr = listener.local_addr().expect("Failed to get server address");
                let thread_name = format!("{}:{}/{}:{}",
                                          client_addr.ip(), client_addr.port(),
                                          server_addr.ip(), server_addr.port());
                info!("New echo connection: {}", thread_name);
                thread::Builder::new().name(thread_name).spawn(move || {
                    echo_connection_handler(stream);
                }).expect("Failed to spawn thread");
            }
            Err(e) => {
                error!("Error accepting echo connection: {}", e);
            }
        }
    }
}