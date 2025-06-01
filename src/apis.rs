use strum::FromRepr;

#[repr(i16)]
#[derive(Copy, Clone, Debug, FromRepr)]
pub enum ApiKey {
    ApiVersions = 18,
    DescribeTopicPartitions = 75,
}
