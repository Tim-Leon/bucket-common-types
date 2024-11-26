use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// BucketGuid is a combination between user_id and bucket_id.
// Max character length of 63 for aws s3 bucket name https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BucketGuid {
    pub user_id: uuid::Uuid,
    pub bucket_id: uuid::Uuid,
}

/// Enum to specify different formats for displaying/parsing BucketGuid,
/// It will use the specified underlying UUID format, but will combine the UUIDs in different format depending on whether it is using
/// Hyphenated or Simple
pub enum BucketGuidFormat {
    Hyphenated(UuidFormat),
    Simple(UuidFormat),
}

/// Enum to specify different formats for displaying/parsing UUID.
pub enum UuidFormat {
    Hyphenated,
    Simple,
    Braced,
    Urn,
}

impl UuidFormat {
    pub fn fmt_uuid(&self, f: &mut fmt::Formatter<'_>, uuid: &Uuid) -> fmt::Result {
        match self {
            UuidFormat::Hyphenated => write!(f, "{}", uuid.hyphenated()),
            UuidFormat::Simple => write!(f, "{}", uuid.simple()),
            UuidFormat::Braced => write!(f, "{}", uuid.braced()),
            UuidFormat::Urn => write!(f, "{}", uuid.urn()),
        }
    }
}

impl fmt::Display for BucketGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Default display format: Hyphenated
        self.fmt_with(f, BucketGuidFormat::Hyphenated(UuidFormat::Simple))
    }
}
impl BucketGuid {
    /// Returns a 32-byte array representation of the BucketGuid.
    pub fn to_bytes(&self) -> [u8; 32] {
            let mut slice = [0u8; 32];
            slice[0..16].copy_from_slice(self.user_id.as_bytes());
            slice[16..32].copy_from_slice(self.bucket_id.as_bytes());
            slice
    }

    /// Format the BucketGuid using the specified format.
    pub fn fmt_with(&self, f: &mut fmt::Formatter<'_>, format: BucketGuidFormat) -> fmt::Result {
        match format {
            BucketGuidFormat::Hyphenated(uuid_format) => {
                uuid_format.fmt_uuid(f, &self.user_id)?;
                write!(f, "-")?;
                uuid_format.fmt_uuid(f, &self.bucket_id)
            }
            BucketGuidFormat::Simple(uuid_format) => {
                uuid_format.fmt_uuid(f, &self.user_id)?;
                uuid_format.fmt_uuid(f, &self.bucket_id)
            }
        }
    }
}


impl BucketGuid {
    pub fn new(user_id: uuid::Uuid, bucket_id: uuid::Uuid) -> Self {
        Self { user_id, bucket_id }
    }

    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
        }
    }

    // Define the size of a ``BucketGuid`` in bytes.
    pub const fn size() -> usize {
        // Since each UUID is 16 bytes, the total length is 32 bytes
        32
    }
}


impl FromStr for BucketGuid {
    type Err = BucketGuidParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // +1 to accommodate for optional hyphen `-` in the middle between user_id and bucket_id.
        if s.len() != BucketGuid::size() || s.len() != (BucketGuid::size() + 1) {
         return Err(BucketGuidParseError::InvalidLength)
        };
        let user_id = Uuid::parse_str(&s[..36]).map_err(BucketGuidParseError::UuidParserFailed)?;
        let bucket_id = Uuid::parse_str(&s[36..]).map_err(BucketGuidParseError::UuidParserFailed)?;
        Ok(Self { user_id, bucket_id })

    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BucketGuidParseError {
    #[error("The length of the input string is invalid.")]
    InvalidLength,

    #[error("Failed to parse UUID: {0}")]
    UuidParserFailed(#[source] uuid::Error),
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_with_different_formats() {
        let user_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let bucket_id = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
        let bucket_guid = BucketGuid::new(user_id, bucket_id);

        // Test Hyphenated format with different UUID formats
        let result = format!("{}", bucket_guid);
        assert_eq!(
            result,
            "550e8400e29b41d4a716446655440000-67e5504410b1426f9247bb680e5fe0c8"
        );

        // Test Simple format
        let result = format!("{}", bucket_guid);
        assert_eq!(
            result,
            "550e8400e29b41d4a716446655440000-67e5504410b1426f9247bb680e5fe0c8"
        );
    }

    #[test]
    fn test_basic_operations() {
        // Test new()
        let user_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let bucket_id = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
        let bucket_guid = BucketGuid::new(user_id, bucket_id);
        assert_eq!(bucket_guid.user_id, user_id);
        assert_eq!(bucket_guid.bucket_id, bucket_id);

        // Test generate()
        let generated = BucketGuid::generate();
        assert_ne!(generated.user_id, generated.bucket_id);

        // Test to_bytes()
        let bytes = bucket_guid.to_bytes();
        assert_eq!(bytes.len(), 32);
        assert_eq!(&bytes[0..16], user_id.as_bytes());
        assert_eq!(&bytes[16..32], bucket_id.as_bytes());
    }

    #[test]
    fn test_size() {
        assert_eq!(BucketGuid::size(), 32);
    }
}