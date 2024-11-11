use crate::region::RegionCluster;
use crate::share::centralized::centralized_secrete_share_link_token::CentralizedSecretShareLinkToken;
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
pub struct CentralizedSecreteShareLink {
    pub scheme: Scheme,
    pub fqdn: FullyQualifiedDomainName,
    pub path: TokenPath,
}


impl TryFrom<CentralizedSecretShareLinkToken> for CentralizedSecreteShareLink {
    type Error = Infallible;

    fn try_from(value: CentralizedSecretShareLinkToken) -> Result<Self, Self::Error> {

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
        Ok(CentralizedSecreteShareLink {
            scheme: Scheme::HTTPS,
            fqdn,
            path,
        })
    }
}
#[derive(thiserror::Error, Debug)]
pub enum CentralizedSecreteShareLinkTokenUriEncodedParseError {
    #[error(transparent)]
    FailedToParse(#[from] http::Error),
}
impl TryInto<Uri> for CentralizedSecreteShareLink {
    type Error = CentralizedSecreteShareLinkTokenUriEncodedParseError;

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
impl TryFrom<Uri> for CentralizedSecreteShareLink {
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

#[cfg(test)]
mod tests {
    use super::*;
    use http::Uri;
    use std::str::FromStr;

    // Test for FullyQualifiedDomainName parsing
    #[test]
    fn test_fully_qualified_domain_name_parsing() {
        let valid_fqdn = "subdomain.example.com";
        let parsed = FullyQualifiedDomainName::from_str(valid_fqdn);
        assert!(parsed.is_ok());
        let fqdn = parsed.unwrap();
        assert_eq!(fqdn.domain, "example.com");
        assert_eq!(fqdn.subdomain, Some("subdomain".to_string()));

        let invalid_fqdn = "example";
        let parsed_invalid = FullyQualifiedDomainName::from_str(invalid_fqdn);
        assert!(parsed_invalid.is_ok());
        let fqdn_invalid = parsed_invalid.unwrap();
        assert_eq!(fqdn_invalid.domain, "example");
        assert!(fqdn_invalid.subdomain.is_none());

        let empty_fqdn = "";
        let parsed_empty = FullyQualifiedDomainName::from_str(empty_fqdn);
        assert!(parsed_empty.is_err());
    }

    // Test for CentralizedSecreteShareLinkTokenUrlEncodedV1 conversion from CentralizedSecretShareLinkToken
    #[test]
    fn test_centralized_secret_share_link_token_urlencoded_v1_conversion() {
        let token = CentralizedSecretShareLinkToken {
            token: "test_token".to_string(),
            region: Some(RegionCluster {
                region: "us-west".to_string(),
            }),
        };

        let result: CentralizedSecreteShareLink = token.try_into().unwrap();
        assert_eq!(result.scheme, Scheme::HTTPS);
        assert_eq!(result.fqdn.domain, DOMAIN_URL.to_string());
        assert_eq!(result.fqdn.subdomain, Some("us-west".to_string()));
        assert_eq!(result.path.version.to_string(), SharingApiPath::V1.to_string());
    }

    // Test for TryInto<Uri> conversion for CentralizedSecreteShareLinkTokenUrlEncodedV1
    #[test]
    fn test_centralized_secrete_share_link_token_urlencoded_v1_to_uri() {
        let token = CentralizedSecretShareLinkToken {
            token: "test_token".to_string(),
            region: Some(RegionCluster {
                region: "us-west".to_string(),
            }),
        };

        let link_url_encoded: CentralizedSecreteShareLink = token.try_into().unwrap();
        let uri: Uri = link_url_encoded.try_into().unwrap();

        assert_eq!(uri.scheme_str(), Some("https"));
        assert_eq!(uri.host(), Some("us-west.example.com"));
        assert_eq!(uri.path(), "/share/v1/test_token");
    }

    // Test error handling for TryFrom<Uri> for invalid FQDN
    #[test]
    fn test_try_from_uri_invalid_fqdn() {
        let uri = Uri::from_str("https://invalid_fqdn.com/share/v1/token").unwrap();
        let result: Result<CentralizedSecreteShareLink, _> = uri.try_into();
        assert!(result.is_err());
        if let Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidFqdnFormat) = result {
            // Expected error
        } else {
            panic!("Expected InvalidFqdnFormat error");
        }
    }

    // Test error handling for TryFrom<Uri> for invalid scheme
    #[test]
    fn test_try_from_uri_invalid_scheme() {
        let uri = Uri::from_str("ftp://us-west.example.com/share/v1/token").unwrap();
        let result: Result<CentralizedSecreteShareLink, _> = uri.try_into();
        assert!(result.is_err());
        if let Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidScheme) = result {
            // Expected error
        } else {
            panic!("Expected InvalidScheme error");
        }
    }

    // Test for invalid path in TryFrom<Uri>
    #[test]
    fn test_try_from_uri_invalid_path() {
        let uri = Uri::from_str("https://us-west.example.com/invalid_path").unwrap();
        let result: Result<CentralizedSecreteShareLink, _> = uri.try_into();
        assert!(result.is_err());
        if let Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidPathFormat) = result {
            // Expected error
        } else {
            panic!("Expected InvalidPathFormat error");
        }
    }
}
