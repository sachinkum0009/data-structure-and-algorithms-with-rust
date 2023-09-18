use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                let message = String::from_utf8_lossy(&buffer[0..size]);
                println!("Received: {}", message);

                // Add a timestamp to the response and terminate with a newline character
                let response = format!("Received: {} at {:?}\n", message, Instant::now());
                stream.write_all(response.as_bytes()).unwrap();
                true
            }
        }
        Err(_) => {
            println!("Error reading from client.");
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
