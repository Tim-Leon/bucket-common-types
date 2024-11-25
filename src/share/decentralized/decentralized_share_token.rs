use std::convert::TryFrom;
use std::ops::Deref;

use aes_gcm::aes::cipher::crypto_common::OutputSizeUser;
use digest::generic_array::{ArrayLength, GenericArray};
use digest::Digest;
use secrecy::ExposeSecret;
use time::OffsetDateTime;

use crate::bucket::{bucket_feature_flags::BucketFeaturesFlags, bucket_guid::BucketGuid};
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::key::derived_key::DerivedKey;
use crate::region::DatacenterRegion;
use crate::share::share_link_token::ShareLinkToken;

#[derive(Clone, Debug)]
pub struct DecentralizedSecretShareToken {
    pub token: ShareLinkToken,
    pub region: Option<DatacenterRegion>,
}

#[derive(thiserror::Error, Debug)]
pub enum DecentralizedSecretShareTokenError {
    #[error("Bucket feature flag does not support decentralized sharing")]
    MissingDecentralizedSharableFeature,
    #[error("Failed to serialize expiration time: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("Failed to create token from hash: {0}")]
    TokenConversionError(#[from] std::array::TryFromSliceError),
}

impl DecentralizedSecretShareToken {
    /// Generates a cryptographic hash for the decentralized secret sharing token.
    pub fn hash<TDigest: Digest + OutputSizeUser>(
        region_cluster: &Option<DatacenterRegion>,
        bucket_guid: &BucketGuid,
        secret_key: &Option<DerivedKey>,
        permission: &BucketPermissionFlags,
        expires: &OffsetDateTime,
    ) -> Result<GenericArray<u8, TDigest::OutputSize>, bincode::Error>
     where <TDigest as OutputSizeUser>::OutputSize: generic_array::ArrayLength {
        let mut hasher = TDigest::new();
        if let Some(region) = region_cluster {
            hasher.update(region.to_string());
        }
        hasher.update(bucket_guid.to_bytes());
        match secret_key {
            Some(secret_key) =>  {
                hasher.update(secret_key.key.expose_secret());
            },
            None => { },
        };
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(expires)?);
        Ok(hasher.finalize())
    }

    /// Creates a new `DecentralizedSecretShareToken` instance.
    pub fn new<TDigest, TKeyLength>(
        region_cluster: &Option<DatacenterRegion>,
        bucket_guid: &BucketGuid,
        secret_key: &Option<DerivedKey>,
        permission: &BucketPermissionFlags,
        expires: &OffsetDateTime,
        bucket_features_flags: &BucketFeaturesFlags,
    ) -> Result<Self, DecentralizedSecretShareTokenError>
    where
        TDigest: Digest + OutputSizeUser,
        TKeyLength: ArrayLength<u8>,
        <TDigest as OutputSizeUser>::OutputSize: generic_array::ArrayLength
    {
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_DECENTRALIZED_SHARABLE) {
            return Err(DecentralizedSecretShareTokenError::MissingDecentralizedSharableFeature);
        }

        let token_hash = Self::hash::<TDigest>(
            region_cluster,
            bucket_guid,
            secret_key,
            permission,
            expires,
        )?;

        Ok(Self {
            token: ShareLinkToken(<[u8; 32]>::try_from(token_hash.as_slice())?),
            region: region_cluster.clone(),
        })
    }
}

impl Deref for DecentralizedSecretShareToken {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.token.0.as_slice()
    }
}
