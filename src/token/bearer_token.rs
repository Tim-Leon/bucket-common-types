use std::fmt::Display;
use crate::authentication::BearerToken;


#[derive(Clone, Debug, PartialEq)]
#[derive(Zeroize)]
pub struct BearerToken(String);
impl BearerToken {
    pub fn to_bearer_token(&self) -> String {
        format!("Bearer {}", self.0)
    }
}
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
        write!(f, "{}", self.0)
    }
}

