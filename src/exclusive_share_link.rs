use crate::secret_share_link::SecretShareLink;
use crate::share_link::ShareLink;
use std::fmt;
use std::fmt::{Display, Formatter};

// Just a enum used to store share link.
#[cfg(feature = "share_link")]
#[cfg(feature = "secret_share_link")]
pub enum ExclusiveShareLink {
    ShareLink(ShareLink),
    SecretShareLink(SecretShareLink),
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
impl TryFrom<url::Url> for ExclusiveShareLink {
    type Error = ExclusiveShareLinkParsingError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        match ShareLink::try_from(value.clone()) {
            Ok(share_link) => {
                return Ok(ExclusiveShareLink::ShareLink(share_link));
            }
            Err(_) => {}
        };
        match SecretShareLink::try_from(value) {
            Ok(secret_share_link) => {
                return Ok(ExclusiveShareLink::SecretShareLink(secret_share_link));
            }
            Err(_) => {}
        };
        Err(ExclusiveShareLinkParsingError::FailedToParse)
    }
}

#[cfg(feature = "share_link")]
#[cfg(feature = "secret_share_link")]
impl Display for ExclusiveShareLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExclusiveShareLink::ShareLink(share_link) => {
                write!(f, "{}", share_link)
            }
            ExclusiveShareLink::SecretShareLink(secret_share_link) => {
                write!(f, "{}", secret_share_link)
            }
        }
    }
}
