
/*
* The encryption has version control built in
* The format is Encryption:Version,
* None: uses no encryption.
* AES256: uses server side encryption.
* Zero-Knowledge: uses client side encryption.
* Custom: uses custom encryption. Relies on the client implementing the encryption specifics.
*/
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use crate::Role;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
pub enum EncryptionAlgorithm {
    None,
    AES256,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
    // Must start with 'Custom-' and then the name of the encryption. with a max length of 64 characters entirely.
    #[strum(to_string = "custom-{0}")]
    Custom(String),
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BucketEncryptionScheme {
    /// The version of the encryption scheme implementation.
    /// This field tracks changes in the encryption method over time.
    pub version: u32,

    /// Specifies who is responsible for managing the encryption of the bucket.
    /// The responsible entity can be identified by the `Role` enum (e.g., `Owner`, `CloudProvider`).
    pub responsible: Role,

    /// The encryption algorithm used to secure the data in the bucket.
    /// This is represented by the `EncryptionAlgorithm` enum.
    pub encryption: EncryptionAlgorithm,
}

impl Display for BucketEncryptionScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "{}:{}:{}",
            &self.version,
            &self.responsible,
            &self.encryption,
        )
    }
}

#[derive(thiserror::Error, Debug, Display)]
pub enum BucketEncryptionParsingError {
    InvalidFormat,
    InvalidRole,
    InvalidVersion,
}
//TODO: https://github.com/P3KI/bendy
impl FromStr for BucketEncryptionScheme
{
    type Err = BucketEncryptionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err(BucketEncryptionParsingError::InvalidFormat);
        }

        let role = Role::from_str(parts[0]).map_err(|x| BucketEncryptionParsingError::InvalidRole)?;

        let encryption = parts[1].to_string();
        let signature = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };

        let version = match parts[3].parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err(BucketEncryptionParsingError::InvalidVersion),
        };

        Ok(BucketEncryptionScheme {
            responsible: role,
            encryption: EncryptionAlgorithm::from_str(encryption.as_str()).unwrap(),
            version,
        })
    }
}


#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum EncryptionParsingError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("custom format too long")]
    CustomFormatTooLong,
    #[error("invalid delimiter")]
    InvalidDelimiter,
    #[error(transparent)]
    FailedToParseVersion(#[from] ParseIntError),
}

impl FromStr for EncryptionAlgorithm {
    type Err = EncryptionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let encryption = parts
            .next()
            .ok_or(EncryptionParsingError::InvalidDelimiter)?;
        let encryption = EncryptionAlgorithm::from_str(encryption)?;
        Ok(encryption)
    }
}
#[cfg(test)]
mod bucket_encryption_tests {
    use super::*;

    #[test]
    fn test_validate_bucket_encryption() {
        // Test valid inputs
        assert_eq!(
            EncryptionAlgorithm::from_str("None"),
            Ok(EncryptionAlgorithm::None)
        );

        assert_eq!(
            EncryptionAlgorithm::from_str("AES256"),
            Ok(EncryptionAlgorithm::AES256)
        );


        assert_eq!(
            EncryptionAlgorithm::from_str("custom-MyEncryption"),
            Ok(EncryptionAlgorithm::Custom("custom-MyEncryption".to_string()))
        );

        // Test invalid formats
        assert_eq!(
            EncryptionAlgorithm::from_str("InvalidEncryption"),
            Err(EncryptionParsingError::InvalidFormat)
        );

        assert_eq!(
            EncryptionAlgorithm::from_str("AES256"), // Missing version
            Err(EncryptionParsingError::InvalidDelimiter)
        );
    }

    #[test]
    fn test_valid_bucket_encryption_parsing() {
        assert_eq!(
            "None".parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::None)
        );
        assert_eq!(
            "AES256".parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::AES256)
        );

        assert_eq!(
            "Custom-Test".parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::Custom("Custom-Test".to_string()))
        );
    }

    #[test]
    fn test_invalid_bucket_encryption_parsing() {
        assert!("Invalid".parse::<EncryptionAlgorithm>().is_err());
        assert!("AES256:".parse::<EncryptionAlgorithm>().is_err()); // Missing version
        assert!(":42".parse::<EncryptionAlgorithm>().is_err()); // Missing encryption type
        assert!(
            "Custom-ThisIsAVeryLongStringThatShouldFailToParseWithOver64CharactersXXX"
                .parse::<EncryptionAlgorithm>()
                .is_err()
        ); // Too long custom encryption
    }

    #[test]
    fn test_invalid_version() {
        assert!("AES256:invalid".parse::<EncryptionAlgorithm>().is_err()); // Invalid version
    }

    #[test]
    fn test_custom_encryption_max_length() {
        let long_custom_encryption = format!("Custom-{}", "x".repeat(63)); // Create a custom encryption of max length

        assert_eq!(
            long_custom_encryption.parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::Custom(long_custom_encryption))
        );

        let too_long_custom_encryption = format!("Custom-{}", "x".repeat(65)); // Exceeds max length
        assert!(too_long_custom_encryption
            .parse::<EncryptionAlgorithm>()
            .is_err());
    }
}
