pub mod api_versions;
pub mod apis;
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

        #[error("unssupported API key")]
        UnsupportedApiKeyError,

        #[error("unsupported version")]
        UnsupportedVersionError(#[from] ApiVersionError),
    }

    #[derive(Debug)]
    pub struct KafkaRequest {
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
        pub client_id: Option<String>,
    }

    impl TryFrom<&[u8]> for KafkaRequest {
        type Error = KafkaRequestError;

        fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
            let request_api_key_i16 = i16::from_be_bytes(bytes[0..2].try_into()?);
            let request_api_version = i16::from_be_bytes(bytes[2..4].try_into()?);
            let correlation_id = i32::from_be_bytes(bytes[4..8].try_into()?);

            Ok(KafkaRequest {
                request_api_key: request_api_key_i16,
                request_api_version,
                correlation_id,
                client_id: None,
            })
        }
    }
}

pub mod kafka_response {
    use crate::api_versions;
    use std::convert::From;

    pub trait KafkaResponseSerialize {
        fn to_be_bytes(&self) -> Vec<u8>;
    }

    #[derive(Debug)]
    pub enum KafkaResponseHeader {
        KafkaResponseHeaderV0(KafkaResponseHeaderV0),
    }

    impl KafkaResponseHeader {
        pub fn to_be_bytes(&self) -> Vec<u8> {
            match self {
                KafkaResponseHeader::KafkaResponseHeaderV0(h) => h.to_be_bytes(),
            }
        }
    }

    #[derive(Debug)]
    pub enum KafkaResponseBody {
        ApiVersions(api_versions::api::ResponseBody),
    }

    impl KafkaResponseBody {
        pub fn to_be_bytes(&self) -> Vec<u8> {
            match self {
                KafkaResponseBody::ApiVersions(b) => b.to_be_bytes(),
            }
        }
    }

    #[derive(Debug)]
    pub struct KafkaResponse {
        pub header: KafkaResponseHeader,
        pub body: KafkaResponseBody,
    }

    #[derive(Debug)]
    pub struct KafkaResponseHeaderV0 {
        pub correlation_id: i32,
    }

    impl KafkaResponseSerialize for KafkaResponseHeaderV0 {
        fn to_be_bytes(&self) -> Vec<u8> {
            let mut stream: Vec<u8> = Vec::new();
            stream.extend(self.correlation_id.to_be_bytes());
            stream
        }
    }

    impl From<KafkaResponse> for Vec<u8> {
        fn from(val: KafkaResponse) -> Self {
            let mut stream: Vec<u8> = Vec::new();
            stream.extend(val.header.to_be_bytes());
            stream.extend(val.body.to_be_bytes());

            let message_size: i32 = stream.len().try_into().expect("usize smaller than i32");
            stream.splice(0..0, message_size.to_be_bytes());
            stream
        }
    }
}
