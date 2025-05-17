pub mod kafka_request {
    use std::array::TryFromSliceError;
    use std::convert::TryFrom;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum KafkaRequestError {
        #[error("unreadable bytestream")]
        UnreadableBytestream(#[from] TryFromSliceError),
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
    }

    impl From<KafkaResponse> for Vec<u8> {
        fn from(val: KafkaResponse) -> Self {
            [val.message_size, val.correlation_id]
                .iter()
                .flat_map(|&v| v.to_be_bytes())
                .collect()
        }
    }
}
