use core::slice::SlicePattern;
use digest::{Digest, OutputSizeUser};
use digest::generic_array::GenericArray;
use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::region::RegionCluster;
use crate::share::decentralized::decentralized_secrete_share_link_url_encoded::DecentralizedSecretShareLink;
use crate::share::decentralized::decentralized_secrete_share_token::DecentralizedSecretShareToken;
use crate::share::share_link_token::{SecreteShareLinkToken, ShareLinkToken};

pub struct DecentralizedShareToken {
    pub token: ShareLinkToken,
    pub region: Option<RegionCluster>,
}

impl DecentralizedShareToken {
    pub fn hash<TDigest: Digest + OutputSizeUser>(
        bucket_guid: &BucketGuid,
        permission: &BucketPermissionFlags,
        expires: &OffsetDateTime,
    ) -> GenericArray<u8, <TDigest as OutputSizeUser>::OutputSize> {
        let mut output = GenericArray::default();
        let mut hasher = TDigest::new();
        hasher.update(bucket_guid.as_slice());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires).unwrap());
        hasher.finalize_into(&mut output);
        output
    }
    pub fn new(decentralized_secret_share_link: &DecentralizedSecretShareLink) -> Self {
        let token = Self::hash(&decentralized_secret_share_link.bucket_guid,
                               &decentralized_secret_share_link.permission,
                               &decentralized_secret_share_link.expires);
        Self {
            token: SecreteShareLinkToken(token),
            region: None,
        }
    }
}