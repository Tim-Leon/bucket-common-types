use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::BucketPermission;
use crate::region::RegionCluster;

#[derive(Clone)]
pub enum UrlEncodedShareLinkVersion {

}

/// All the information is encoded into a URL, Note that differing from the SecreteShareLinkUrlEncoded, this does not contain an encryption key.
#[derive(Clone, PartialEq, Eq)]
pub struct ShareLinkUrlEncoded {
    pub version: UrlEncodedShareLinkVersion,

    pub region_cluster: RegionCluster,

    pub bucket_guid: BucketGuid,
    /// The permission associated with the url.
    pub permission: BucketPermission,
    /// For how long the signature is going to be valid
    pub expires: OffsetDateTime,
    // The signature is stored in the link. This makes sure that the link is not tampered with.
    pub signature: ed25519_compact::Signature,
}

impl ShareLinkUrlEncoded {
    const VERSION: &'static u8 = 1;
    pub fn new(bucket_guid: BucketGuid, expires: OffsetDateTime, permission: BucketSharePermissionFlags ,secret_signing_key: &ed25519_compact::SecretKey) -> Self {
        let hash = self::comute_hash(bucket_guid, permission, expires);
        let signature = secret_signing_key.sign(hash);
        Self {
           version: self::VERSION,
            bucket_guid,
            expires,
            permission,
            signature,
        }
    }

    pub fn compute_hash<THasher: Digest + OutputSizeUser>(
        bucket_guid: BucketGuid,
        permission: BucketSharePermissionFlags,
        expires: OffsetDateTime,
    ) -> GenericArray<u8, <THasher as OutputSizeUser>::OutputSize> {
        let output = GenericArray::default();
        let mut hasher = THasher::new();
        hasher.update(bucket_guid.as_bytes());
        hasher.update(permission.bits().to_be_bytes());
        hasher.update(bincode::serialize(&expires).unwrap());
        hasher.finalize_into(output)
    }

}