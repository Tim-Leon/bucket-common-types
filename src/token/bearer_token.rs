use std::fmt::Display;
use zeroize::Zeroize;
use crate::token::jwt_token::JwtToken;

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Zeroize)]
pub struct BearerToken(pub String);
impl TryFrom<&str> for BearerToken {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            0: value.to_string(),
        })
    }
}

impl From<JwtToken> for BearerToken {
    fn from(value: JwtToken) -> Self {
        Self { 0: value }
    }
}

impl Display for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bearer {}", self.0)
    }
}

