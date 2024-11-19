use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::key::CryptoHashDerivedKeyType;
use crate::region::RegionCluster;
use crate::share::share_link_token::SecreteShareLinkToken;
use aes_gcm::aes::cipher::crypto_common::OutputSizeUser;
use core::slice::SlicePattern;
use digest::generic_array::GenericArray;
use digest::Digest;
use ed25519_compact::{Noise, PublicKey, SecretKey};
use generic_array::ArrayLength;
use time::OffsetDateTime;
use crate::share::decentralized::decentralized_share_token::TokenSignature;

#[derive(Clone, Debug)]
pub struct DecentralizedSecretShareToken {
    pub token: SecreteShareLinkToken,
    pub region: Option<RegionCluster>,
}

impl DecentralizedSecretShareToken
{

    pub fn hash<TDigest: Digest + OutputSizeUser, TKeyLength : ArrayLength>(
        region_cluster: &Option<RegionCluster>,
        bucket_guid: &BucketGuid,
        bucket_key: &impl CryptoHashDerivedKeyType<TKeyLength>,
        permission: &BucketPermissionFlags,
        expires: &OffsetDateTime,
    ) -> GenericArray<u8, <TDigest as OutputSizeUser>::OutputSize> {
        let mut hasher = TDigest::new();
        let mut output = GenericArray::default();
        match region_cluster {
            None => {}
            Some(region_cluster) => {
                hasher.update(region_cluster.to_string());
            }
        };
        hasher.update(bucket_guid.to_bytes());
        hasher.update(bucket_key.as_slice());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires).unwrap());
        hasher.finalize_into(&mut output);
        output
    }
    /// Computes the decentralized secret sharing token. The token is used for setting permissions for a bucket without sharing the secrete. Unlike centralized sharing
    /// The share link is generated from the token.
    pub fn new<TKeyLength: generic_array::ArrayLength>(
                        region_cluster: &Option<RegionCluster>,
                       bucket_guid: &BucketGuid,
                       bucket_key: &impl CryptoHashDerivedKeyType<TKeyLength>,
                       permission: &BucketPermissionFlags,
                       expires: &OffsetDateTime) -> Self{
        let mut token = Self::hash(&region_cluster,
                                   &bucket_guid,
                                   &bucket_key.as_slice(),
                                   &permission,
                                   &expires);
        Self {
            token:  SecreteShareLinkToken(<[u8; 32]>::try_from(token.as_slice()).unwrap()),
            region: region_cluster.clone(),
        }
    }


    pub fn sign(&self, secrete_key: &SecretKey, bucket_guid: &BucketGuid) -> TokenSignature {
        //let noise = Noise::from_slice(self.region);
        let noise = Noise::from_slice(&bucket_guid.to_bytes()).unwrap();
        TokenSignature(secrete_key.sign(&self.token.0.as_slice(),Some(noise)))
    }

    pub fn verify(&self, public_key: &PublicKey, signature: &TokenSignature) -> Result<(), ed25519_compact::Error> {
        public_key.verify(self.token.0.as_slice(), &signature.0)
    }
}