use crate::region::RegionCluster;
use crate::share::centralized::centralized_secrete_share_link_token::CentralizedSecretShareLinkToken;
use crate::share::versioning::SharingApiPath;
use crate::util::DOMAIN_URL;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use http::uri::Scheme;
use http::Uri;
use std::convert::Infallible;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;
use crate::share::fully_qualified_domain_name::FullyQualifiedDomainName;
use crate::share::share_link_token::ShareLinkToken;

/// path/version/token, serialize and deserializer.
pub struct PathWithToken {
    pub version: SharingApiPath,
    pub token: ShareLinkToken,
}

impl Display for PathWithToken {
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
    #[error(transparent)]
    InvalidVersionFormat,
    #[error(transparent)]
    MissingToken,
    #[error(transparent)]
    InvalidSharePrefix,
    #[error(transparent)]
    InvalidTokenFormat(#[from] ),
}

impl FromStr for PathWithToken {
    type Err = PathVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure the string contains the "/share/" prefix
        if !s.starts_with(SharingApiPath::V1.to_string()) {
            return Err(PathVersionParseError::InvalidSharePrefix);
        }

        // Find the start of the path after "/share/"
        let path = &s[share_prefix.len()..];

        // Split by "/" to get version and token
        let parts: Vec<&str> = path.split('/').collect();

        // We expect exactly 2 parts: version and token
        if parts.len() != 2 {
            return Err(PathVersionParseError::InvalidFormat); // Expect exactly 2 parts
        }

        // Parse the version
        let version = SharingApiPath::from_str(parts[0])
            .map_err(|_| PathVersionParseError::InvalidVersionFormat)?;

        // Ensure the token is present
        let token = URL_SAFE_NO_PAD.decode(parts[1]).unwrap();
        if token.is_empty() {
            return Err(PathVersionParseError::MissingToken);
        }
        // Return the successfully parsed PathVersion
        Ok(PathWithToken { version, token: token.as_slice().try_into().unwrap() })
    }
}

/// Would use the URL create to store the data but the URL crate does not have a Builder Pattern only parser, very sad.
pub struct CentralizedSecreteShareLink {
    pub scheme: Scheme,
    pub fqdn: FullyQualifiedDomainName,
    pub path: PathWithToken,
}


impl TryFrom<CentralizedSecretShareLinkToken> for CentralizedSecreteShareLink {
    type Error = Infallible;

    fn try_from(value: CentralizedSecretShareLinkToken) -> Result<Self, Self::Error> {

        // Version and token encoding
        let path = PathWithToken {
            version: SharingApiPath::V1,
            token: value.token,
        };
        
        let fqdn = FullyQualifiedDomainName {
            subdomain: value.region.map(|x| x.region.to_string()),
            domain: DOMAIN_URL.to_string(),
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
        let path = PathWithToken::from_str(value.path())
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
