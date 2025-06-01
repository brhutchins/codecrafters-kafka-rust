use crate::apis::ApiKey;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiVersionError {
    #[error("Unspported version")]
    UnsupportedVersionError(String),
}

#[derive(Debug)]
pub enum ApiVersion {
    V4,
}

impl TryFrom<i16> for ApiVersion {
    type Error = ApiVersionError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        println!("API Version: {:?}", value);
        match value {
            4 => Ok(ApiVersion::V4),
            val => Err(ApiVersionError::UnsupportedVersionError(val.to_string())),
        }
    }
}

impl From<ApiVersion> for i16 {
    fn from(value: ApiVersion) -> Self {
        match value {
            ApiVersion::V4 => 4,
        }
    }
}

impl ApiVersion {
    pub fn from_be_bytes(value: [u8; 2]) -> Result<Self, ApiVersionError> {
        let value_i16 = i16::from_be_bytes(value);

        value_i16.try_into()
    }
}

#[derive(Clone, Debug)]
pub struct ApiVersionInfo {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
    pub tag_buffer: i8,
}

impl ApiVersionInfo {
    const fn new(api_key: ApiKey, min_version: i16, max_version: i16) -> Self {
        Self {
            api_key,
            min_version,
            max_version,
            tag_buffer: 0,
        }
    }

    fn to_be_bytes(&self) -> Vec<u8> {
        let mut stream: Vec<u8> = Vec::new();
        let api_key = self.api_key as i16;
        stream.extend(api_key.to_be_bytes());
        stream.extend(self.min_version.to_be_bytes());
        stream.extend(self.max_version.to_be_bytes());
        stream.extend(self.tag_buffer.to_be_bytes());
        stream
    }
}

pub static API_VERSIONS: [ApiVersionInfo; 2] = [
    ApiVersionInfo::new(ApiKey::ApiVersions, 0, 4),
    ApiVersionInfo::new(ApiKey::DescribeTopicPartitions, 0, 4),
];

pub mod api {
    use super::*;
    use crate::kafka_response::KafkaResponseSerialize;

    #[derive(Debug)]
    pub struct ResponseBody {
        pub error_code: i16,
        pub api_versions: Vec<ApiVersionInfo>,
        pub throttle_time: i32,
        pub tag_buffer: i8,
    }

    impl ResponseBody {
        pub fn new(error_code: i16) -> Self {
            Self {
                error_code,
                api_versions: API_VERSIONS.to_vec(),
                throttle_time: 0,
                tag_buffer: 0,
            }
        }
    }

    impl KafkaResponseSerialize for ResponseBody {
        fn to_be_bytes(&self) -> Vec<u8> {
            let mut stream: Vec<u8> = Vec::new();
            stream.extend(self.error_code.to_be_bytes());

            let array_length = self.api_versions.len() as i8 + 1;
            stream.extend(array_length.to_be_bytes());
            for v in self.api_versions.iter() {
                stream.extend(v.to_be_bytes());
            }
            stream.extend(self.throttle_time.to_be_bytes());
            stream.extend(self.tag_buffer.to_be_bytes());
            stream
        }
    }
}
