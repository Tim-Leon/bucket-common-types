use std::convert::Infallible;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use base64::alphabet::URL_SAFE;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use http::Uri;
use http::uri::Scheme;
use url::Url;
use crate::region::RegionCluster;
use crate::share::centralized::centralized_secrete_share_link_token::CentralizedSecretShareLinkToken;
use crate::share::versioning::UrlEncodedShareLinksVersioning;
use crate::util::DOMAIN_URL;

#[derive(Debug)]
pub struct FullyQualifiedDomainName {
    subdomain: Option<String>,
    domain: String,
}

impl Display for FullyQualifiedDomainName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Full host including subdomain if present
        match &self.subdomain {
            None => write!(f, "{}", self.domain),  // Just the domain
            Some(subdomain) => write!(f, "{}.{}", subdomain, self.domain),  // Subdomain + domain
        }
    }
}


/// Error type for FullyQualifiedDomainName parsing.
#[derive(thiserror::Error, Debug)]
pub enum FullyQualifiedDomainNameParseError {
    #[error("Invalid domain name format")]
    InvalidFormat,
}

impl FromStr for FullyQualifiedDomainName {
    type Err = FullyQualifiedDomainNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(FullyQualifiedDomainNameParseError::InvalidFormat);
        }

        // Split the input string by the '.' character
        let parts: Vec<&str> = s.split('.').collect();

        // If there are at least two parts, treat the first as subdomain and the rest as domain
        if parts.len() > 1 {
            let subdomain = Some(parts[0].to_string());
            let domain = parts[1..].join(".");
            Ok(FullyQualifiedDomainName {
                subdomain,
                domain,
            })
        } else if parts.len() == 1 {
            // If only one part is provided, it's a domain without subdomain
            Ok(FullyQualifiedDomainName {
                subdomain: None,
                domain: parts[0].to_string(),
            })
        } else {
            // If the string is empty or doesn't contain valid parts, return an error
            Err(FullyQualifiedDomainNameParseError::InvalidFormat)
        }
    }
}

pub struct PathVersion {
    pub version: UrlEncodedShareLinksVersioning,
    pub token: String,
}


impl Display for PathVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "/share/{}/{}", self.version.to_string(), self.token)
    }
}


// Define a custom error type for PathVersion parsing
#[derive(Debug, thiserror::Error)]
pub enum PathVersionParseError {
    InvalidFormat,
    InvalidVersionFormat,
    MissingToken,
    InvalidSharePrefix,
}

impl FromStr for PathVersion {
    type Err = PathVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure the string contains the "/share/" prefix
        let share_prefix = "/share/";
        if !s.starts_with(share_prefix) {
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
        let version = UrlEncodedShareLinksVersioning::from_str(parts[0])
            .map_err(|_| PathVersionParseError::InvalidVersionFormat)?;

        // Ensure the token is present
        let token = parts[1];

        if token.is_empty() {
            return Err(PathVersionParseError::MissingToken);
        }

        // Return the successfully parsed PathVersion
        Ok(PathVersion { version, token })
    }
}

/// Would use the URL create to store the data but the URL crate does not have a Builder Pattern only parser, very sad.
pub struct CentralizedSecreteShareLinkTokenUrlEncodedV1 {
    pub scheme: Scheme,
    pub fqdn: FullyQualifiedDomainName,
    pub path: PathVersion,
}


impl TryFrom<CentralizedSecretShareLinkToken> for CentralizedSecreteShareLinkTokenUrlEncodedV1 {
    type Error = Infallible;

    fn try_from(value: CentralizedSecretShareLinkToken) -> Result<Self, Self::Error> {

        // Version and token encoding
        let version = UrlEncodedShareLinksVersioning::V1;
        let token = URL_SAFE_NO_PAD.encode(&value.token);
    
        let path = PathVersion {
            version,
            token,
        };
        
        let fqdn = FullyQualifiedDomainName {
            subdomain: value.region.map(|x| x.region.to_string()),
            domain: DOMAIN_URL.to_string(),
        };

        // Return the constructed struct
        Ok(CentralizedSecreteShareLinkTokenUrlEncodedV1 {
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
impl TryInto<Uri> for CentralizedSecreteShareLinkTokenUrlEncodedV1 {
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
impl TryFrom<Uri> for CentralizedSecreteShareLinkTokenUrlEncodedV1 {
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
        let path = PathVersion::from_str(value.path())
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
    use std::str::FromStr;
    use http::Uri;

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

        let result: CentralizedSecreteShareLinkTokenUrlEncodedV1 = token.try_into().unwrap();
        assert_eq!(result.scheme, Scheme::HTTPS);
        assert_eq!(result.fqdn.domain, DOMAIN_URL.to_string());
        assert_eq!(result.fqdn.subdomain, Some("us-west".to_string()));
        assert_eq!(result.path.version.to_string(), UrlEncodedShareLinksVersioning::V1.to_string());
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

        let link_url_encoded: CentralizedSecreteShareLinkTokenUrlEncodedV1 = token.try_into().unwrap();
        let uri: Uri = link_url_encoded.try_into().unwrap();

        assert_eq!(uri.scheme_str(), Some("https"));
        assert_eq!(uri.host(), Some("us-west.example.com"));
        assert_eq!(uri.path(), "/share/v1/test_token");
    }

    // Test error handling for TryFrom<Uri> for invalid FQDN
    #[test]
    fn test_try_from_uri_invalid_fqdn() {
        let uri = Uri::from_str("https://invalid_fqdn.com/share/v1/token").unwrap();
        let result: Result<CentralizedSecreteShareLinkTokenUrlEncodedV1, _> = uri.try_into();
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
        let result: Result<CentralizedSecreteShareLinkTokenUrlEncodedV1, _> = uri.try_into();
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
        let result: Result<CentralizedSecreteShareLinkTokenUrlEncodedV1, _> = uri.try_into();
        assert!(result.is_err());
        if let Err(CentralizedSecreteShareLinkTokenUrlEncodedError::InvalidPathFormat) = result {
            // Expected error
        } else {
            panic!("Expected InvalidPathFormat error");
        }
    }
}
