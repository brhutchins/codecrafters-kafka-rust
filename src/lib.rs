pub mod api_versions;
pub mod errors;

pub mod kafka_request {
    use crate::api_versions::ApiVersionError;
    use std::array::TryFromSliceError;
    use std::convert::TryFrom;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum KafkaRequestError {
        #[error("unreadable bytestream")]
        UnreadableBytestream(#[from] TryFromSliceError),

        #[error("unsupported version")]
        UnsupportedVersionError(#[from] ApiVersionError),
    }

    #[derive(Debug)]
    pub struct KafkaRequest {
        pub message_size: i32,
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
        pub client_id: Option<String>,
    }

    impl TryFrom<&[u8]> for KafkaRequest {
        type Error = KafkaRequestError;

        fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
            let message_size = i32::from_be_bytes(bytes[0..4].try_into()?);
            let request_api_key = i16::from_be_bytes(bytes[4..6].try_into()?);
            let request_api_version = i16::from_be_bytes(bytes[6..8].try_into()?);
            let correlation_id = i32::from_be_bytes(bytes[8..12].try_into()?);

            Ok(KafkaRequest {
                message_size,
                request_api_key,
                request_api_version,
                correlation_id,
                client_id: None,
            })
        }
    }
}

pub mod kafka_response {
    use std::convert::From;

    #[derive(Debug)]
    pub struct KafkaResponse {
        pub message_size: i32,
        pub correlation_id: i32,
        pub error_code: i16,
    }

    impl From<KafkaResponse> for Vec<u8> {
        fn from(val: KafkaResponse) -> Self {
            let mut stream: Vec<u8> = Vec::new();
            stream.extend(val.message_size.to_be_bytes());
            stream.extend(val.correlation_id.to_be_bytes());
            stream.extend(val.error_code.to_be_bytes());
            stream
        }
    }
}
