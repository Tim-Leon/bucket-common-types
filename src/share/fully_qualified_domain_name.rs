use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct FullyQualifiedDomainName {
    pub subdomain: Option<String>,
    pub domain: String,
}

pub struct Domain {
    pub domain_name: String,
    pub tld: String,
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