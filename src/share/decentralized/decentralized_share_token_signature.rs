use core::slice::SlicePattern;
use std::convert::Infallible;

use ed25519_compact::{Noise, PublicKey, SecretKey, Signature};

use crate::{bucket::bucket_guid::BucketGuid, share::share_link_token::ShareLinkToken};

use super::decentralized_share_token::DecentralizedSecretShareToken;


#[derive(thiserror::Error, Debug)]
pub enum DecentralizedShareTokenSignatureError {
    #[error("Noise creation error: {0:?}")]
    FailedToCreateNoise(#[source] ed25519_compact::Error),

    #[error("Signature parsing error: {0:?}")]
    FailedToParseSlice(#[source] ed25519_compact::Error),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct DecentralizedShareTokenSignature(pub Signature);

impl DecentralizedShareTokenSignature {
    pub fn new(
        token: &DecentralizedSecretShareToken,
        secret_key: &SecretKey,
        bucket_guid: &BucketGuid,
    ) -> Result<Self, DecentralizedShareTokenSignatureError> {
        let noise = Noise::from_slice(&bucket_guid.to_bytes())
            .map_err(DecentralizedShareTokenSignatureError::FailedToCreateNoise)?;
        let siganture = secret_key.sign(token.as_slice(), Some(noise));
        
        Ok(
            Self {
                0: siganture
            }
        )
    }

    pub fn from_slice(signature: &[u8]) -> Result<Self, DecentralizedShareTokenSignatureError> {
        Signature::from_slice(signature)
            .map(Self)
            .map_err(DecentralizedShareTokenSignatureError::FailedToParseSlice)
    }

    pub fn verify(
        &self,
        public_key: &PublicKey,
        token: &ShareLinkToken,
    ) -> Result<(), ed25519_compact::Error> {
        public_key.verify(&token.0, &self.0)
    }
}
