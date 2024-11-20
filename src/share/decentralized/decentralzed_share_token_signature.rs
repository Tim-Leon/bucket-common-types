use ed25519_compact::{Noise, PublicKey, SecretKey, Signature};

use crate::{bucket::bucket_guid::BucketGuid, share::share_link_token::ShareLinkToken};

use super::{decentralized_share_token::DecentralizedShareToken, decentralized_share_tokens_union::DecentralizedSecreteShareTokenUnion};


#[derive(thiserror::Error, Debug)]
pub enum DecentralizedShareTokenSignatureError {
    #[error(transparent)]
    FailedToCreateNoise(#[from] ed25519_compact::Error), 
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DecentralizedShareTokenSignature(pub Signature);


impl DecentralizedShareTokenSignature {
    pub fn new(token: &DecentralizedSecreteShareTokenUnion,secrete_key: &SecretKey, bucket_guid: &BucketGuid) -> Result<Self, DecentralizedShareTokenSignatureError> {
        let noise = Noise::from_slice(&bucket_guid.to_bytes())?;
        match &token {
            DecentralizedSecreteShareTokenUnion::DecentralizedShareToken(dst) =>  {
                Ok(Self(secrete_key.sign(&dst.0.as_slice(),Some(noise))))
            },
            DecentralizedSecreteShareTokenUnion::DecentralizedSecretShareToken(dsst) =>  {
                Ok(Self(secrete_key.sign(&dsst.0.as_slice(),Some(noise))))
            },
        }

    }

    pub fn verify(&self, public_key: &PublicKey, token: &ShareLinkToken) -> Result<(), ed25519_compact::Error>{
        public_key.verify(token.0, &self.0)
    }
}