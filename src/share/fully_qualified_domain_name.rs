use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents a fully qualified domain name with up to 3 levels using owned, boxed strings.
#[derive(Debug)]
pub struct FullyQualifiedDomainName {
    pub subdomain: Option<Box<str>>,
    pub domain: Box<str>,
    pub top_level_domain: Box<str>,
}

impl Display for FullyQualifiedDomainName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.subdomain {
            Some(subdomain) => write!(f, "{}.{}.{}", subdomain, self.domain, self.top_level_domain),
            None => write!(f, "{}.{}", self.domain, self.top_level_domain),
        }
    }
}

/// Error type for FullyQualifiedDomainName parsing.
#[derive(thiserror::Error, Debug)]
pub enum FullyQualifiedDomainNameParseError {
    #[error("Empty string provided")]
    EmptyString,
    #[error("Invalid domain name format")]
    InvalidFormat,
}

impl FromStr for FullyQualifiedDomainName {
    type Err = FullyQualifiedDomainNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(FullyQualifiedDomainNameParseError::EmptyString);
        }

        // Split the input string by '.' character
        let parts: Vec<&str> = s.split('.').collect();

        match parts.as_slice() {
            [subdomain, domain, top_level_domain] => Ok(FullyQualifiedDomainName {
                subdomain: Some(Box::from(*subdomain)),
                domain: Box::from(*domain),
                top_level_domain: Box::from(*top_level_domain),
            }),
            [domain, top_level_domain] => Ok(FullyQualifiedDomainName {
                subdomain: None,
                domain: Box::from(*domain),
                top_level_domain: Box::from(*top_level_domain),
            }),
            _ => Err(FullyQualifiedDomainNameParseError::InvalidFormat),
        }
    }
}
