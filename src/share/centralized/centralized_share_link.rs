use crate::share::centralized::centralized_share_link_token::CentralizedShareLinkToken;
use crate::share::fully_qualified_domain_name::FullyQualifiedDomainName;
use crate::share::token_path::TokenPath;
use crate::share::versioning::SharingApiPath;
use crate::util::{DOMAIN_NAME, DOMAIN_URL};
use base64::Engine;
use http::uri::Scheme;
use http::Uri;
use std::convert::Infallible;
use std::fmt::Display;
use std::io::BufRead;
use std::str::FromStr;

/// Would use the URL create to store the data but the URL crate does not have a Builder Pattern only parser, very sad.
pub struct CentralizedShareLink {
    pub scheme: Scheme,
    pub fqdn: FullyQualifiedDomainName,
    pub path: TokenPath,
}

impl TryFrom<CentralizedShareLinkToken> for CentralizedShareLink {
    type Error = Infallible;

    fn try_from(value: CentralizedShareLinkToken) -> Result<Self, Self::Error> {

        // Version and token encoding
        let path = TokenPath {
            version: SharingApiPath::V1,
            token: value.token,
        };

        let subdomain = match value.region {
            None => { None }
            Some(region) => { Some( Box::from( region.to_string().as_str())) }
        };

        let fqdn = FullyQualifiedDomainName {
            subdomain,
            domain: Box::from(DOMAIN_NAME),
            top_level_domain: Box::from(".co"),
        };

        // Return the constructed struct
        Ok(CentralizedShareLink {
            scheme: Scheme::HTTPS,
            fqdn,
            path,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedShareLinkTokenUriEncodedParseError {
    #[error(transparent)]
    FailedToParse(#[from] http::Error),
}
impl TryInto<Uri> for CentralizedShareLink {
    type Error = CentralizedShareLinkTokenUriEncodedParseError;

    fn try_into(self) -> Result<Uri, Self::Error> {
        // Create URI, handling errors with `?`
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(self.fqdn.to_string())
            .path_and_query(self.path.to_string())
            .build()?;
        Ok(uri)
    }
}


// Custom error enum for try_from
#[derive(thiserror::Error, Debug)]
pub enum CentralizedSecreteShareLinkTokenUrlEncodedError {
    #[error("Invalid URI scheme")]
    InvalidScheme,

    #[error("Invalid FQDN format")]
    InvalidFqdnFormat,

    #[error("Invalid path format")]
    InvalidPathFormat,
}
impl TryFrom<Uri> for CentralizedShareLink {
    type Error = CentralizedSecreteShareLinkTokenUrlEncodedError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        // Handle scheme
        let scheme = match value.scheme() {
            None => return Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidScheme),
            Some(scheme) => scheme.clone(),
        };

        // Handle host (FQDN)
        let fqdn = match value.host() {
            None => return Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidFqdnFormat),
            Some(host) => FullyQualifiedDomainName::from_str(host)
                .map_err(|_| CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidFqdnFormat)?,
        };

        // Handle path
        let path = TokenPath::from_str(value.path())
            .map_err(|_| CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidPathFormat)?;

        // Construct and return the struct
        Ok(Self {
            scheme,
            fqdn,
            path,
        })
    }
}
