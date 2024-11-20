use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::region::RegionCluster;
use crate::share::share_link_token::{SecreteShareLinkToken, ShareLinkToken};
use crate::token;
use core::slice::SlicePattern;
use digest::generic_array::GenericArray;
use digest::{Digest, OutputSizeUser};
use ed25519_compact::{Noise, PublicKey, SecretKey, Signature};
use sha3::{Sha3_256, Sha3_256Core};
use time::OffsetDateTime;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DecentralizedShareToken {
    pub token: ShareLinkToken,
    pub region: Option<RegionCluster>,
}


impl DecentralizedShareToken {
    pub fn hash<TDigest: Digest + OutputSizeUser>(
        bucket_guid: &BucketGuid,
        permission: &BucketPermissionFlags,
        expires_at: &OffsetDateTime,
    ) -> GenericArray<u8, <TDigest as OutputSizeUser>::OutputSize> {
        let mut output = GenericArray::default();
        let mut hasher = TDigest::new();
        hasher.update(bucket_guid.to_bytes());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires_at).unwrap());
        hasher.finalize_into(&mut output);
        output
    }
    pub fn new(bucket_guid: &BucketGuid,
               permission: &BucketPermissionFlags,
               expires_at: &OffsetDateTime,
                region: &Option<RegionCluster>) -> Self {
        let token = Self::hash::<Sha3_256>(&bucket_guid,
                               &permission,
                               &expires_at);
        assert_eq!(token.len(), 32);
        Self {
            token: ShareLinkToken(<[u8; 32]>::try_from(token.as_slice()).unwrap()),
            region: *region,
        }
    }
}