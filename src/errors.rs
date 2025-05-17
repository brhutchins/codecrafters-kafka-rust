use crate::api_versions::ApiVersionError;
use thiserror::Error;

#[derive(Error, Debug)]
#[repr(i16)]
pub enum KafkaError {
    #[error("Unspported version")]
    UnsupportedVersionError(#[from] ApiVersionError),
}

impl KafkaError {
    pub fn code(&self) -> i16 {
        match self {
            KafkaError::UnsupportedVersionError(_) => 35,
        }
    }
}
