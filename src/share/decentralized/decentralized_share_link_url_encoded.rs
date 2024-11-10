use core::slice::SlicePattern;
use digest::generic_array::GenericArray;
use digest::{Digest, OutputSizeUser};
use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::region::RegionCluster;
use crate::share::versioning::UrlEncodedShareLinksVersioning;

/// All the information is encoded into a URL, Note that differing from the SecreteShareLinkUrlEncoded, this does not contain an encryption key.
#[derive(Clone, PartialEq, Eq)]
pub struct DecentralizedShareLinkUrlEncoded {
    pub version: UrlEncodedShareLinksVersioning,

    pub region: Option<RegionCluster>,

    pub bucket_guid: BucketGuid,
    /// The permission associated with the url.
    pub permission: BucketPermissionFlags,
    /// For how long the signature is going to be valid
    pub expires: OffsetDateTime,
    // The signature is stored in the link. This makes sure that the link is not tampered with.
    pub signature: ed25519_compact::Signature,
}

impl DecentralizedShareLinkUrlEncoded {
    const VERSION: UrlEncodedShareLinksVersioning = UrlEncodedShareLinksVersioning::V1;
    pub fn new(region: Option<RegionCluster>,bucket_guid: BucketGuid, expires: OffsetDateTime, permission: BucketPermissionFlags ,secret_signing_key: &ed25519_compact::SecretKey) -> Self {
        let hash = Self::compute_hash(bucket_guid, permission, expires);
        let signature = secret_signing_key.sign(hash);
        Self {
           version: Self::VERSION,
            region: region,
            bucket_guid,
            expires,
            permission,
            signature,
        }
    }



}