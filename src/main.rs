use codecrafters_kafka::{
    api_versions,
    api_versions::ApiVersion,
    errors::KafkaError,
    kafka_request::KafkaRequest,
    kafka_response::{
        KafkaResponse, KafkaResponseBody, KafkaResponseHeader, KafkaResponseHeaderV0,
    },
};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").expect("Failed to start TcpListener");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    let _ = handle_stream(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = bytes::BytesMut::new();

    loop {
        let mut read_buffer = [0u8; 4096];
        match stream.read(&mut read_buffer) {
            Ok(0) => {
                println!("Client closed the connection");
                break;
            }
            Ok(n) => buffer.extend_from_slice(&read_buffer[..n]),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e),
        };

        while buffer.len() >= 4 {
            let msg_length =
                i32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
            let total_length = msg_length + 4;

            if buffer.len() < total_length {
                // If we haven't receive the whole message yet, we'll break to the next iteration
                // of the loop, and try again.
                println!("Haven't received full message. Looping again");
                break;
            }

            let msg = buffer.split_to(total_length);
            let request: KafkaRequest = msg[4..].try_into().expect("Failed to parse KafkaRequest");
            println!("{:?}", request);

            let mut error_code: i16 = 0;

            match handle_request(&request) {
                Ok(v) => v,
                Err(err) => {
                    println!("{:?}", err);
                    error_code = err.code();
                }
            };
            let response_header = KafkaResponseHeaderV0 {
                correlation_id: request.correlation_id,
            };

            let response_body = api_versions::api::ResponseBody::new(error_code);

            let response = KafkaResponse {
                header: KafkaResponseHeader::KafkaResponseHeaderV0(response_header),
                body: KafkaResponseBody::ApiVersions(response_body),
            };

            println!("Response: {:?}", response);

            let response_bytes: Vec<u8> = response.into();

            println!("Response bytes: {:02x?}", response_bytes);

            stream.write_all(&response_bytes).unwrap();
        }
    }

    Ok(())
}

fn handle_request(request: &KafkaRequest) -> Result<(), KafkaError> {
    let _api_version: ApiVersion = request.request_api_version.try_into()?;
    Ok(())
}
