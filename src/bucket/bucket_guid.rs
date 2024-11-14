use core::slice::SlicePattern;
use std::{fmt, mem};
use std::str::FromStr;
use logos::Source;
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

//match format {
//    BucketGuidFormat::Hyphenated(uuid_format) => write!(f, "{}-{}", self.user_id, self.bucket_id),
//    BucketGuidFormat::Simple(uuid_format) => write!(
//        f,
//        "{}{}",
//        self.user_id,
//        self.bucket_id
//    ),
//}
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
        let size:usize = 32;
        debug_assert_eq!(size, mem::size_of::<BucketGuid>());
        size
    }
}

impl SlicePattern for BucketGuid {
    type Item = u8;
    /// 8-bit array collection of 32 items.
    fn as_slice(&self) -> &[Self::Item] {
        let mut slice = [0u8; 32];
        slice[0..16].copy_from_slice(self.user_id.as_bytes());
        slice[16..32].copy_from_slice(self.bucket_id.as_bytes());
        &slice
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

    // Test the `new` method to create a new BucketGuid
    #[test]
    fn test_new() {
        let user_id = Uuid::new_v4();
        let bucket_id = Uuid::new_v4();
        let bucket_guid = BucketGuid::new(user_id, bucket_id);

        assert_eq!(bucket_guid.user_id, user_id);
        assert_eq!(bucket_guid.bucket_id, bucket_id);
    }

    // Test the `generate` method to create a random BucketGuid
    #[test]
    fn test_generate() {
        let bucket_guid = BucketGuid::generate();

        // Ensure that the generated GUID has different user_id and bucket_id
        assert_ne!(bucket_guid.user_id, bucket_guid.bucket_id);
    }

    // Test the `size` constant to verify that BucketGuid's size is 32 bytes
    #[test]
    fn test_size() {
        assert_eq!(BucketGuid::size(), 32);
    }

    // Test the `as_slice` method to ensure it correctly converts to a slice of bytes
    #[test]
    fn test_as_slice() {
        let user_id = Uuid::new_v4();
        let bucket_id = Uuid::new_v4();
        let bucket_guid = BucketGuid::new(user_id, bucket_id);

        let slice = bucket_guid.as_slice();
        assert_eq!(slice.len(), 32); // Ensure the slice is 32 bytes
        assert_eq!(&slice[0..16], user_id.as_bytes()); // First 16 bytes are user_id
        assert_eq!(&slice[16..32], bucket_id.as_bytes()); // Last 16 bytes are bucket_id
    }

    // Test the `Display` implementation to ensure it formats correctly
    #[test]
    fn test_display() {
        let user_id = Uuid::new_v4();
        let bucket_id = Uuid::new_v4();
        let bucket_guid = BucketGuid::new(user_id, bucket_id);

        let formatted = format!("{}", bucket_guid);
        let expected = format!("{}-{}", user_id.as_simple(), bucket_id.as_simple());

        assert_eq!(formatted, expected);
    }

    // Test the `FromStr` implementation with valid input
    #[test]
    fn test_from_str_valid() {
        let user_id = Uuid::new_v4();
        let bucket_id = Uuid::new_v4();
        let input = format!("{}-{}", user_id, bucket_id);

        let parsed = BucketGuid::from_str(&input);
        assert!(parsed.is_ok());
        let bucket_guid = parsed.unwrap();
        assert_eq!(bucket_guid.user_id, user_id);
        assert_eq!(bucket_guid.bucket_id, bucket_id);
    }

    // Test the `FromStr` implementation with invalid input (wrong length)
    #[test]
    fn test_from_str_invalid_length() {
        let input = "invalid-length-uuid";
        let parsed = BucketGuid::from_str(input);

        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err(), BucketGuidParseError::InvalidLength);
    }

    // Test the `FromStr` implementation with invalid UUID format
    #[test]
    fn test_from_str_invalid_uuid() {
        let input = "invalid-uuid-format-invalid-uuid";
        let parsed = BucketGuid::from_str(input);

        assert!(parsed.is_err());
        match parsed.unwrap_err() {
            BucketGuidParseError::UuidParserFailed(_) => {} // Expected error
            _ => panic!("Expected UuidParserFailed error"),
        }
    }
}
