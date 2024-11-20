use core::slice::SlicePattern;
use std::convert::Infallible;
use digest::generic_array::GenericArray;
use digest::{Digest, OutputSizeUser};
use http::uri::Scheme;
use time::OffsetDateTime;
use crate::bucket::bucket_guid::BucketGuid;
use crate::bucket::bucket_permission::BucketPermissionFlags;
use crate::region::RegionCluster;
use crate::share::decentralized::decentralized_share_token::{DecentralizedShareToken};
use crate::share::versioning::SharingApiPath;

use super::decentralized_share_tokens_union::DecentralizedSecreteShareTokenUnion;
use super::decentralzed_share_token_signature::DecentralizedShareTokenSignature;

/// All the information is encoded into a URL, Note that differing from the SecreteShareLinkUrlEncoded, this does not contain an encryption key.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecentralizedShareLink {
    pub scheme: Scheme,
    pub region: Option<RegionCluster>,
    pub version: SharingApiPath,
    pub bucket_guid: BucketGuid,
    /// The permission associated with the url.
    pub permission: BucketPermissionFlags,
    /// For how long the signature is going to be valid
    pub expires: OffsetDateTime,
    /// Token
    pub token: DecentralizedShareToken,
    // The signature is stored in the link. This makes sure that the link is not tampered with.
    pub signature: DecentralizedShareTokenSignature,
}


impl DecentralizedShareLink {
    const VERSION: SharingApiPath = SharingApiPath::V1;
    pub fn new(region: Option<RegionCluster>, bucket_guid: BucketGuid, expires: OffsetDateTime, permission: BucketPermissionFlags ,secret_signing_key: &ed25519_compact::SecretKey) -> Result<Self, Infallible> {
        let token = DecentralizedShareToken::new(&bucket_guid, &permission, &expires, &region);
        let token_union =  DecentralizedSecreteShareTokenUnion::DecentralizedShareToken(token.clone());
        let token_signature = DecentralizedShareTokenSignature::new(&token_union, &secret_signing_key, &bucket_guid).unwrap(); 
        Ok(Self {
            scheme: Scheme::HTTPS,
            version: Self::VERSION,
            region,
            bucket_guid,
            expires,
            permission,
            signature: token_signature,
            token,
        })
    }

    pub fn get_token(&self) -> DecentralizedShareToken {
        self.token.clone()
    }
}