use codecrafters_kafka::{kafka_request::KafkaRequest, kafka_response::KafkaResponse};
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
    let mut buffer = [0; 39];
    stream
        .read_exact(&mut buffer[..])
        .expect("Failed to read stream");
    let request: KafkaRequest = buffer[..].try_into().expect("Failed to parse KafkaRequest");
    println!("{:?}", request);

    let message_size: i32 = 10;
    let response = KafkaResponse {
        message_size,
        correlation_id: request.correlation_id,
    };

    println!("{:?}", response);

    let response_bytes: Vec<u8> = response.into();

    stream.write_all(&response_bytes).unwrap();
}
