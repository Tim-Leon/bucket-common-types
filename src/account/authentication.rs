use std::fmt::Display;
use crate::token::bearer_token::BearerToken;

pub enum AuthenticationScheme {
    /// Not supported.
    Basic(String),
    /// Not supported.
    Digest(String),
    Bearer(BearerToken),
}

impl Display for AuthenticationScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AuthenticationScheme::Basic(s) => write!(f, "Basic {}", s),
            AuthenticationScheme::Digest(s) => write!(f, "Digest {}", s),
            AuthenticationScheme::Bearer(s) => write!(f, "Bearer {}", s),
        }
    }
}

