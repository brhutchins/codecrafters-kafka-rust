use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").expect("Failed to start TcpListener");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_stream(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let message_size: i32 = 0;
    let correlation_id: i32 = 7;
    let response: Vec<u8> = [message_size, correlation_id]
        .iter()
        .flat_map(|&v| v.to_be_bytes())
        .collect();

    stream.write_all(&response).unwrap();
}
