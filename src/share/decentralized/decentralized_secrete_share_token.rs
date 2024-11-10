use core::slice::SlicePattern;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::Aes256Gcm;
use aes_gcm::aes::cipher::crypto_common::OutputSizeUser;
use digest::Digest;
use digest::generic_array::GenericArray;
use rand::CryptoRng;
use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::key::CryptoHashDerivedKeyType;
use crate::region::RegionCluster;
use crate::share::decentralized::decentralized_secrete_share_link_url_encoded::DecentralizedSecretShareLink;

pub struct DecentralizedSecretShareToken {
    pub token: [u8;32],
    pub region: Option<RegionCluster>,
}

impl DecentralizedSecretShareToken
{

    pub fn create_token_secrete_share_link<TDigest: Digest + OutputSizeUser, TKeyLength>(
        region_cluster: &RegionCluster,
        bucket_guid: &BucketGuid,
        bucket_key: &impl CryptoHashDerivedKeyType<TKeyLength>,
        permission: &BucketPermissionFlags,
        expires: &OffsetDateTime,
    ) -> GenericArray<u8, <TDigest as OutputSizeUser>::OutputSize> {
        let mut hasher = TDigest::new();
        let mut output = GenericArray::default();
        hasher.update(region_cluster.to_string());
        hasher.update(bucket_guid.as_slice());
        hasher.update(bucket_key.as_slice());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires).unwrap());
        hasher.finalize_into(&mut output);
        output
    }
    /// Computes the decentralized secret sharing token. The token is used for setting permissions for a bucket without sharing the secrete. Unlike centralized sharing
    /// The share link is generated from the token.
    pub fn new(secrete_share_link: &DecentralizedSecretShareLink) -> Self{
        let mut token = Self::create_token_secrete_share_link(&secrete_share_link.region_cluster,
                                                              &secrete_share_link.bucket_guid,
                                                              &secrete_share_link.bucket_key,
                                                              &secrete_share_link.permission,
                                                              &secrete_share_link.expires);
        Self {
            token: <[u8; 32]>::try_from(token.as_slice()).unwrap(),
            region: secrete_share_link.region_cluster.clone(),
        }
    }
}