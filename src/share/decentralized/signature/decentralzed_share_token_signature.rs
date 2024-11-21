use std::convert::Infallible;

use ed25519_compact::{Noise, PublicKey, SecretKey, Signature};

use crate::{bucket::bucket_guid::BucketGuid, share::{decentralized::decentralized_share_tokens_union::DecentralizedSecretShareTokenUnion, share_link_token::ShareLinkToken}};


#[derive(thiserror::Error, Debug)]
pub enum DecentralizedShareTokenSignatureError {
    #[error("Noise creation error: {0:?}")]
    FailedToCreateNoise(#[source] ed25519_compact::Error),

    #[error("Signature parsing error: {0:?}")]
    FailedToParseSlice(#[source] ed25519_compact::Error),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DecentralizedShareTokenSignature(pub Signature);

impl DecentralizedShareTokenSignature {
    pub fn new(
        token: &DecentralizedSecretShareTokenUnion,
        secret_key: &SecretKey,
        bucket_guid: &BucketGuid,
    ) -> Result<Self, DecentralizedShareTokenSignatureError> {
        let noise = Noise::from_slice(&bucket_guid.to_bytes())
            .map_err(DecentralizedShareTokenSignatureError::FailedToCreateNoise)?;
        match token {
            DecentralizedSecretShareTokenUnion::DecentralizedShareToken(dst) => {
                Ok(Self(secret_key.sign(dst.0.as_slice(), Some(noise))))
            }
            DecentralizedSecretShareTokenUnion::DecentralizedSecretShareToken(dsst) => {
                Ok(Self(secret_key.sign(dsst.0.as_slice(), Some(noise))))
            }
        }
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
