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
