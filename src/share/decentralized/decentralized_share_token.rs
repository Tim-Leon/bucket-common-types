use core::slice::SlicePattern;

use crate::bucket::{bucket_feature_flags::BucketFeaturesFlags, bucket_guid::BucketGuid};
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::key::CryptoHashDerivedKeyType;
use crate::region::DatacenterRegion;
use crate::share::share_link_token::SecreteShareLinkToken;
use aes_gcm::aes::cipher::crypto_common::OutputSizeUser;
use digest::generic_array::GenericArray;
use digest::Digest;
use generic_array::ArrayLength;
use time::OffsetDateTime;


#[derive(Clone, Debug)]
pub struct DecentralizedSecretShareToken {
    pub token: SecreteShareLinkToken,
    pub region: Option<DatacenterRegion>,
}

#[derive(thiserror::Error, Debug)]
pub enum DecentralizedSecreteShareTokenError {
    #[error("MissingDecentralizedSharabledFromBucketFeature")]
    MissingDecentralizedSharabledFromBucketFeature,
}

impl DecentralizedSecretShareToken
{

    pub fn hash<TDigest: Digest + OutputSizeUser, TKeyLength : ArrayLength>(
        region_cluster: &Option<DatacenterRegion>,
        bucket_guid: &BucketGuid,
        secrete_key: &impl CryptoHashDerivedKeyType<TKeyLength>,
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
        hasher.update(secrete_key.as_slice());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires).unwrap());
        hasher.finalize_into(&mut output);
        output
    }
    /// Computes the decentralized secret sharing token. The token is used for setting permissions for a bucket without sharing the secrete. Unlike centralized sharing
    /// The share link is generated from the token.
    pub fn new<TKeyLength: generic_array::ArrayLength>(
                        region_cluster: &Option<DatacenterRegion>,
                       bucket_guid: &BucketGuid,
                       secrete_key: &impl CryptoHashDerivedKeyType<TKeyLength>,
                       permission: &BucketPermissionFlags,
                       expires: &OffsetDateTime, 
                    bucket_features_flags: &BucketFeaturesFlags) -> Result<Self, DecentralizedSecreteShareTokenError>{
        if bucket_features_flags.contains(BucketFeaturesFlags::IS_DECENTRALIZED_SHARABLE) {
            return Err(DecentralizedSecreteShareTokenError::MissingDecentralizedSharabledFromBucketFeature)
        }
        let mut token = Self::hash(&region_cluster,
                                   &bucket_guid,
                                   &secrete_key.as_slice(),
                                   &permission,
                                   &expires);
        Ok(Self {
            token:  SecreteShareLinkToken(<[u8; 32]>::try_from(token.as_slice()).unwrap()),
            region: region_cluster.clone(),
        })
    }
}


impl SlicePattern for DecentralizedSecretShareToken {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.token.0.as_slice()
    }
}