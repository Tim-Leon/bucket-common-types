
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
use strum::Display;
use crate::Role;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    None,
    AES256(u8),
    ZeroKnowledge(u8),
    // Must start with 'Custom-' and then the name of the encryption. with a max length of 64 characters entirely.
    Custom(String),
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BucketEncryption {
    // Version of the encryption implementation
    pub version: u32,
    /// Who is responsible for the encryption?
    pub responsible: Role,
    // The encryption to be used.
    pub encryption: EncryptionAlgorithm,
}

impl Display for BucketEncryption {
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
impl FromStr for BucketEncryption
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

        Ok(BucketEncryption {
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
    FaieldToParseVersion(#[from] ParseIntError),
}

impl FromStr for EncryptionAlgorithm {
    type Err = EncryptionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let encryption = parts
            .next()
            .ok_or(EncryptionParsingError::InvalidDelimiter)?;
        match encryption {
            "None" => Ok(EncryptionAlgorithm::None),
            "AES256" | "ZeroKnowledge" => {
                let version = u8::from_str(
                    parts
                        .next()
                        .ok_or(EncryptionParsingError::InvalidDelimiter)?,
                )?;
                match encryption {
                    "AES256" => Ok(EncryptionAlgorithm::AES256(version)),
                    "ZeroKnowledge" => Ok(EncryptionAlgorithm::ZeroKnowledge(version)),
                    _ => unreachable!(), // Should not reach here due to match patterns
                }
            }
            x if x.starts_with("Custom-") => {
                if x.len() > 71 {
                    return Err(EncryptionParsingError::CustomFormatTooLong);
                }
                Ok(EncryptionAlgorithm::Custom(s.to_string()))
            }
            _ => Err(EncryptionParsingError::InvalidFormat),
        }
    }
}

impl Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionAlgorithm::None => write!(f, "None"),
            EncryptionAlgorithm::AES256(version) => write!(f, "AES256:{}", version),
            EncryptionAlgorithm::ZeroKnowledge(version) => write!(f, "ZeroKnowledge:{}", version),
            EncryptionAlgorithm::Custom(name) => write!(f, "Custom-{}", name),
        }
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
            EncryptionAlgorithm::from_str("AES256:1"),
            Ok(EncryptionAlgorithm::AES256(1))
        );


        assert_eq!(
            EncryptionAlgorithm::from_str("ZeroKnowledge:2"),
            Ok(EncryptionAlgorithm::ZeroKnowledge(2))
        );

        assert_eq!(
            EncryptionAlgorithm::from_str("Custom-MyEncryption"),
            Ok(EncryptionAlgorithm::Custom("Custom-MyEncryption".to_string()))
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
            "AES256:42".parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::AES256(42))
        );
        assert_eq!(
            "ZeroKnowledge:5".parse::<EncryptionAlgorithm>(),
            Ok(EncryptionAlgorithm::ZeroKnowledge(5))
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
