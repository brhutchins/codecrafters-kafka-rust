use crate::{api_versions::ApiVersionError, kafka_request::KafkaRequestError};
use thiserror::Error;

#[derive(Error, Debug)]
#[repr(i16)]
pub enum KafkaError {
    #[error("Corrupt message")]
    CorruptMessageError(#[from] KafkaRequestError),
    #[error("Unsupported version")]
    UnsupportedVersionError(#[from] ApiVersionError),
}

impl KafkaError {
    pub fn code(&self) -> i16 {
        match self {
            KafkaError::CorruptMessageError(_) => 2,
            KafkaError::UnsupportedVersionError(_) => 35,
        }
    }
}
