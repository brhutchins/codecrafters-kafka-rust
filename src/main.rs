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
    let mut message_size_buffer = [0; 4];
    stream
        .read_exact(&mut message_size_buffer)
        .expect("Failed to read stream");
    let message_size = i32::from_be_bytes(
        message_size_buffer[..]
            .try_into()
            .expect("Failed to get message size"),
    );
    let mut buffer = vec![0u8; message_size.try_into().expect("usize smaller than i32")];
    stream
        .read_exact(&mut buffer[..])
        .expect("Failed to read stream");
    let request: KafkaRequest = buffer[..].try_into().expect("Failed to parse KafkaRequest");
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

fn handle_request(request: &KafkaRequest) -> Result<(), KafkaError> {
    let _api_version: ApiVersion = request.request_api_version.try_into()?;
    Ok(())
}
