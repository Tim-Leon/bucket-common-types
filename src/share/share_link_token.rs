use std::convert::Infallible;
use std::str::FromStr;
use aes_gcm::aead::rand_core::{CryptoRng, RngCore};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{DecodeError, Engine};

#[derive(Debug, PartialEq, Clone)]
pub struct SecreteShareLinkToken(pub [u8; 32]);

impl SecreteShareLinkToken {

    pub fn generate<TCSPRNG: RngCore + CryptoRng>(cspring :&mut TCSPRNG) -> Self {
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Self(token)
    }
    pub fn to_base64_url_safe(&self) -> String {
        URL_SAFE_NO_PAD.encode(&self.0)
    }

    pub fn from_base64_url_safe(encoded: &str) -> Result<Self, DecodeError> {
        let decoded = URL_SAFE_NO_PAD.decode(encoded)?;
        if decoded.len() != 32 {
            return Err(DecodeError::InvalidLength(decoded.len()));
        }
        let mut token = [0u8; 32];
        token.copy_from_slice(&decoded);
        Ok(SecreteShareLinkToken(token))
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ShareLinkToken(pub [u8; 32]);

impl ShareLinkToken {
    pub fn generate<TCSPRNG: RngCore + CryptoRng>(cspring :&mut TCSPRNG) -> Self {
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Self(token)
    }
    pub fn to_base64_url_safe(&self) -> String {
        URL_SAFE_NO_PAD.encode(&self.0)
    }

    pub fn from_base64_url_safe(encoded: &str) -> Result<Self, DecodeError> {
        let decoded = URL_SAFE_NO_PAD.decode(encoded)?;
        if decoded.len() != 32 {
            return Err(DecodeError::InvalidLength(decoded.len()));
        }
        let mut token = [0u8; 32];
        token.copy_from_slice(&decoded);
        Ok(ShareLinkToken(token))
    }
}

#[derive(Debug, PartialEq)]
pub enum ShareLinkTokens {
    /// Used with decentralized sharing, key information and such are not stored on the server but are instead encoded into the URL, and our server has no clue about the URL.
    SecreteShareLinkToken(SecreteShareLinkToken),
    /// Used with centralized sharing, all info is available to the server.
    ShareLinkToken(ShareLinkToken),
}

impl ShareLinkTokens {
    pub fn to_base64_url_safe(&self) -> String {
        match self {
            ShareLinkTokens::SecreteShareLinkToken(token) => token.to_base64_url_safe(),
            ShareLinkTokens::ShareLinkToken(token) => token.to_base64_url_safe(),
        }
    }

    pub fn from_base64_url_safe(encoded: &str, is_secret: bool) -> Result<Self, DecodeError> {
        if is_secret {
            let token = SecreteShareLinkToken::from_base64_url_safe(encoded)?;
            Ok(ShareLinkTokens::SecreteShareLinkToken(token))
        } else {
            let token = ShareLinkToken::from_base64_url_safe(encoded)?;
            Ok(ShareLinkTokens::ShareLinkToken(token))
        }
    }
}