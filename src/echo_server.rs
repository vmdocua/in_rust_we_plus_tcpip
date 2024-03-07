use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn echo_connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed");
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", message);
                if message.trim() == "shutdown" {
                    println!("Shutdown signal received. Closing server.");
                    return; // Exit the loop and close the connection
                }
                if let Err(_) = stream.write_all(&buffer[0..n]) {
                    println!("Error writing to stream");
                    break;
                } else {
                    println!("Echoed: {}", message);
                }
            }
            Err(_) => {
                println!("Error reading from stream");
                break;
            }
        }
    }
}

pub fn echo_server_run(host: &str, port: u16) {
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address)
        .expect(&format!("Failed to bind to address: {}", address));
    println!("Echo server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New echo connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    echo_connection_handler(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting echo connection: {}", e);
            }
        }
    }
}