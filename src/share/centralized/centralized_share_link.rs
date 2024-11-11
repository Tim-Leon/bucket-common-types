use crate::share::centralized::centralized_secrete_share_link::TokenPath;
use crate::share::fully_qualified_domain_name::FullyQualifiedDomainName;
use crate::share::share_link_token::ShareLinkToken;
use crate::share::versioning::SharingApiPath;
use base64::{DecodeError, Engine};
use http::uri::Scheme;
use http::Uri;
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

pub struct CentralizedShareLink {
    pub scheme: Scheme,
    pub fqdn: FullyQualifiedDomainName,
    pub path: TokenPath,

}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedShareLinkUriEncodingError {
    #[error(transparent)]
    FailedToParse(#[from] http::Error),
}
impl TryInto<Uri> for CentralizedShareLink {
    type Error = CentralizedShareLinkUriEncodingError;

    fn try_into(self) -> Result<Uri, Self::Error> {
        // Create URI, handling errors with `?`
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(&self.fqdn)
            .path_and_query(self.path.to_string())
            .build()?;
        Ok(uri)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ShareLinkUrlParsingError {
    #[error("Failed to decode token")]
    DecodeError(#[from] DecodeError),
    #[error("Parse to string error")]
    ParseToString(#[source] Infallible),
    #[error("Invalid token length")]
    InvalidTokenLength(Vec<u8>),
}


impl TryFrom<url::Url> for CentralizedShareLink {
    type Error = ShareLinkUrlParsingError;
    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        let path = url.path();
        let parts: Vec<&str> = path.split('/').collect();

        // Validate path structure and length
        if parts.len() < 2 {
            return Err(ShareLinkUrlParsingError::InvalidPathStructure);
        }

        // Parse token from the path
        let token = ShareLinkToken::from_base64_url_safe(parts[1])?;

        // Parse FQDN from host string, handling possible errors
        let fqdn = FullyQualifiedDomainName::from_str(url.host_str().unwrap_or("")).map_err(|_| ShareLinkUrlParsingError::InvalidPathStructure)?;

        Ok(Self {
            scheme: Scheme::HTTPS,
            fqdn,
            path: TokenPath {
                version: SharingApiPath::V1,
                token,
            },
        })
    }
}
