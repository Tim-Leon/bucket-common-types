use base64::{engine::general_purpose, DecodeError, Engine};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use rand::{CryptoRng, RngCore};
use crate::util::{DOMAIN_URL, SHARE_PATH_URL};


/*
*  Bucket share link
*  bucketdrive.co/api/v1/share/user_id/bucket_id#permissions#expires#signature
*/
pub struct ShareLinkToken {
    pub token: [u8; 32],
}

impl Display for ShareLinkToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/share/{}",
            DOMAIN_URL,
            SHARE_PATH_URL,
            general_purpose::URL_SAFE_NO_PAD.encode(self.token)
        )
    }
}

impl TryInto<url::Url> for ShareLinkToken {
    type Error = url::ParseError;
    fn try_into(self) -> Result<url::Url, Self::Error> {
        url::Url::parse(self.to_string().as_str())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ShareLinkParsingError {
    #[error("Failed to decode token")]
    DecodeError(#[from] DecodeError),
    #[error("Parse to string error")]
    ParseToString(#[source] Infallible),
    #[error("Invalid token length")]
    InvalidTokenLength(Vec<u8>),
}

// Compress Share Link???
//TODO: FIX THIS
// Very strict parser.
impl TryFrom<url::Url> for ShareLinkToken {
    type Error = ShareLinkParsingError;
    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        let path = url.path();
        let parts = path.split('/').take(1).collect::<Vec<&str>>(); // First element should be empty.
        let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(
            parts[1]
                .parse::<String>()
                .map_err(ShareLinkParsingError::ParseToString)?,
        )?;

        Ok(Self {
            token: token
                .try_into()
                .map_err(ShareLinkParsingError::InvalidTokenLength)?,
        })
    }
}

impl ShareLinkToken {
    pub fn new(token: &[u8; 32]) -> ShareLinkToken {
        Self {
            token: *token,
        }
    }
    pub fn get_token(&self) -> [u8; 32] {
        self.token
    }
    pub fn generate<TCryptoRng: RngCore + CryptoRng>(cspring :&mut TCryptoRng) -> Self {
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Self {
            token,
        }
    }
}

