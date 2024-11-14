use crate::bucket::bucket_feature_flags::BucketFeaturesFlags;
use crate::region::RegionCluster;
use crate::share::share_link_token::{ShareLinkToken, ShareLinkTokens};
use std::fmt::Display;


/*
*  Bucket share link
*  bucketdrive.co/api/v1/share/user_id/bucket_id#permissions#expires#signature
*/
#[derive(Debug)]
pub struct CentralizedShareLinkToken {
    pub token: ShareLinkTokens,
    pub region: Option<RegionCluster>,
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedShareLinkTokenGeneratorError {
    #[error("Centralized shareable feature is not enabled for this bucket")]
    BucketFeatureCentralizedShareableNotEnabled,
}



impl CentralizedShareLinkToken {
    pub fn new(token: ShareLinkTokens, region: Option<RegionCluster>, bucket_features_flags: &BucketFeaturesFlags) -> Result<Self, CentralizedShareLinkTokenGeneratorError> {
        // Check if the bucket feature IS_CENTRALIZED_SHARABLE is enabled
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_CENTRALIZED_SHARABLE) {
            return Err(CentralizedShareLinkTokenGeneratorError::BucketFeatureCentralizedShareableNotEnabled);
        }
        Ok(Self {
            token,
            region,
        })
    }
}

