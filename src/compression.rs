use serde::Deserialize;
// Whenever a bucket is created, compression is set to one of these values or potentially none.
use crate::Serialize;

/// Custom compression is also supported but requires the developer to implement the required traits.
#[derive(
    Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub enum BucketCompression {
    Gzip,
    Brotli,
    Zstd,
    Lz4,
    Custom(String),
}

