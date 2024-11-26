use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")] 
pub enum EncryptionAlgorithm {
    None,
    Aes256,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
    // Must start with 'custom-' and then the name of the encryption. with a max length of 64 characters entirely.
    #[strum(serialize = "custom-{0}")]
    Custom(String),
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
        if s.starts_with("custom-") {
            if s.len() > 64 {
                return Err(EncryptionParsingError::CustomFormatTooLong);
            }
            return Ok(EncryptionAlgorithm::Custom(s.to_string()));
        }

        match s.to_lowercase().as_str() {
            "none" => Ok(EncryptionAlgorithm::None),
            "aes-256" => Ok(EncryptionAlgorithm::Aes256),
            "cha-cha-20-poly-1305" => Ok(EncryptionAlgorithm::ChaCha20Poly1305),
            "x-cha-cha-20-poly-1305" => Ok(EncryptionAlgorithm::XChaCha20Poly1305),
            _ => Err(EncryptionParsingError::InvalidFormat),
        }
    }
}

impl EncryptionAlgorithm {
    // Define constants for OIDs
    const AES256_OID: &'static str = "2.16.840.1.101.3.4.1.46";

    // Use a normal function instead of const fn
    pub fn oid(&self) -> Option<&'static str> {
        match self {
            EncryptionAlgorithm::None => None,
            EncryptionAlgorithm::Aes256 => Some(Self::AES256_OID),
            EncryptionAlgorithm::ChaCha20Poly1305 => None,
            EncryptionAlgorithm::XChaCha20Poly1305 => None,
            EncryptionAlgorithm::Custom(_) => None,
        }
    }
}