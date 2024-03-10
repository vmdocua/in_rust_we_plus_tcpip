// tests/echo_server_test.rs

mod echo_server_test {
    use std::io::{Write, Read};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;
    use in_rust_we_plus_tcpip::echo_server::echo_server_run;

    #[test]
    fn test_echo_server() {
        // Spawn a worker thread for running the echo server
        let _handle = thread::spawn(|| {
            echo_server_run("127.0.0.1", 5535);
        });

        // Wait for the echo server to start
        thread::sleep(Duration::from_secs(1));

        // Establish a TCP connection to the echo server
        let mut stream = TcpStream::connect("127.0.0.1:5535").expect("Failed to connect to server");

        // Write to the server
        stream.write_all(b"Hello").expect("Failed to write to server");

        // Read from the server
        let mut buffer = [0; 5];
        stream.read_exact(&mut buffer).expect("Failed to read from server");

        // Verify the response
        assert_eq!(&buffer, b"Hello");

        // Run thread in background yet
        //handle.join().expect("Failed to join worker thread");
    }
}