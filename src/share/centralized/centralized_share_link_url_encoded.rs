
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use base64::{DecodeError, Engine};
use base64::engine::general_purpose;
use crate::region::RegionCluster;
use crate::share::centralized::centralized_share_link_token::CentralizedShareLinkToken;
use crate::share::versioning::UrlEncodedShareLinksVersioning;
use crate::util::{DOMAIN_URL, SHARE_PATH_URL};


pub struct CentralizedShareLinkUrlEncoded {
    pub subdomain : Option<String>,
    pub domain: String,
    pub version: UrlEncodedShareLinksVersioning,

}

impl Display for CentralizedShareLinkToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}/share/{}",
            DOMAIN_URL,
            SHARE_PATH_URL,
            self.version,
            general_purpose::URL_SAFE_NO_PAD.encode(self.token)
        )
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


impl TryFrom<url::Url> for CentralizedShareLinkToken {
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
            region: self.region,
        })
    }
}
