use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    println!("Connected to server.");

    loop {
        let mut input = String::new();
        println!("Enter a message (or 'quit' to exit): ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit" {
            break;
        }

        let start_time = Instant::now();
        stream.write_all(input.as_bytes()).expect("Failed to write to server");

        let mut response = String::new();
        let mut buf = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buf).expect("Failed to read from server");
            if bytes_read == 0 {
                // No more data to read
                break;
            }
            response.push_str(&String::from_utf8_lossy(&buf[0..bytes_read]));
            if response.contains('\n') {
                // Response contains a newline character, so we've received a complete response
                break;
            }
        }

        let end_time = Instant::now();
        let rtt = end_time - start_time;

        println!("Server response: {}", response.trim());
        println!("RTT: {:?}", rtt);
    }

    println!("Connection closed.");
}
