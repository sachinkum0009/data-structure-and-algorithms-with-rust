use std::io::{Read, Write};
use std::net::TcpStream;

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

        stream.write_all(input.as_bytes()).expect("Failed to write to server");

        let mut response = String::new();
        stream.read_to_string(&mut response).expect("Failed to read from server");
        println!("Server response: {}", response);
    }

    println!("Connection closed.");
}
