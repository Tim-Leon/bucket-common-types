use std::fmt;
use std::fmt::{Display, Formatter};
use crate::share::secrete_share_link_url_encoded::SecretShareLinkUrlEncoded;
use crate::share::share_link_url_encoded::ShareLinkUrlEncoded;

// Just a enum used to store share link.
pub enum ExclusiveShareLinkUrlEncoded {
    ShareLink(ShareLinkUrlEncoded),
    SecretShareLink(SecretShareLinkUrlEncoded),
}

pub enum ExclusiveTokenShareLink {
    TokenShareLink(TokenShareLink),
    SecreteTokenShareLink(SecreteTokenShareLink),
}

pub enum ShareLink {

}

#[cfg(feature = "share_link")]
#[cfg(feature = "secret_share_link")]
#[derive(thiserror::Error, Debug)]
pub enum ExclusiveShareLinkParsingError {
    #[error("Failed to parse")]
    FailedToParse,
}

#[cfg(feature = "share_link")]
#[cfg(feature = "secret_share_link")]
impl TryFrom<url::Url> for ExclusiveShareLinkUrlEncoded {
    type Error = ExclusiveShareLinkParsingError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        match ShareLinkUrlEncoded::try_from(value.clone()) {
            Ok(share_link) => {
                return Ok(ExclusiveShareLinkUrlEncoded::ShareLink(share_link));
            }
            Err(_) => {}
        };
        match SecretShareLinkUrlEncoded::try_from(value) {
            Ok(secret_share_link) => {
                return Ok(ExclusiveShareLinkUrlEncoded::SecretShareLink(secret_share_link));
            }
            Err(_) => {}
        };
        Err(ExclusiveShareLinkParsingError::FailedToParse)
    }
}

#[cfg(feature = "share_link")]
#[cfg(feature = "secret_share_link")]
impl Display for ExclusiveShareLinkUrlEncoded {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExclusiveShareLinkUrlEncoded::ShareLink(share_link) => {
                write!(f, "{}", share_link)
            }
            ExclusiveShareLinkUrlEncoded::SecretShareLink(secret_share_link) => {
                write!(f, "{}", secret_share_link)
            }
        }
    }
}


