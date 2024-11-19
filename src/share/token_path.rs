use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use crate::share::share_link_token::{ShareLinkToken, ShareLinkTokens};
use crate::share::versioning::SharingApiPath;

/// path/version/token, serialize and deserializer.
pub struct TokenPath {
    pub version: SharingApiPath,
    pub token: ShareLinkTokens,
}

impl Display for TokenPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let encoded_token = self.token.to_base64_url_safe();
        write!(f, "{}/{}",self.version.to_string(), encoded_token)
    }
}

// Define a custom error type for PathVersion parsing
#[derive(Debug, thiserror::Error)]
pub enum PathVersionParseError {
    #[error("InvalidFormat")]
    InvalidFormat,
    #[error("InvalidVersionFormat")]
    InvalidVersionFormat,
    #[error("MissingToken")]
    MissingToken,
    #[error("InvalidSharePrefix")]
    InvalidSharePrefix,
}

impl FromStr for TokenPath {
    type Err = PathVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure the string contains the "/share/" prefix
        let prefix = SharingApiPath::V1.to_string();
        if !s.starts_with(&prefix) {
            return Err(PathVersionParseError::InvalidSharePrefix);
        }

        // Find the start of the path after "/share/"
        let path = &s[prefix.len()..];

        // Split by "/" to get version and token
        let parts: Vec<&str> = path.split('/').collect();

        // We expect exactly 2 parts: version and token
        if parts.len() != 2 {
            return Err(PathVersionParseError::InvalidFormat); // Expect exactly 2 parts
        }

        // Parse the version
        let version = SharingApiPath::from_str(parts[0])
            .map_err(|_| PathVersionParseError::InvalidVersionFormat)?;
        // Return the successfully parsed PathVersion
        Ok(TokenPath { version, token: ShareLinkTokens::ShareLinkToken(ShareLinkToken::from_base64_url_safe(parts[1]).unwrap()) })
    }
}
