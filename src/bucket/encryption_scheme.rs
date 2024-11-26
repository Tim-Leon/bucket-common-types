
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

use super::encryption_algorithm::EncryptionAlgorithm;



#[derive(EnumString, PartialEq, Debug, Serialize, strum::Display, Clone, Eq, Deserialize)]
#[repr(u8)]
pub enum Role {
    #[strum(serialize = "S")]
    Server,
    #[strum(serialize = "C")]
    Client,
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

#[cfg(test)]
mod bucket_encryption_tests {
    use crate::bucket::encryption_algorithm::EncryptionParsingError;

    use super::*;


}
