use crate::bucket::bucket_feature_flags::BucketFeaturesFlags;
use crate::region::DatacenterRegion;
use crate::share::share_link_token::ShareLinkTokenUnion;


/*
*  Bucket share link
*  bucketdrive.co/api/v1/share/user_id/bucket_id#permissions#expires#signature
*/
#[derive(Debug)]
pub struct CentralizedShareLinkToken {
    pub token: ShareLinkTokenUnion,
    pub region: Option<DatacenterRegion>,
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedShareLinkTokenGeneratorError {
    #[error("Centralized shareable feature is not enabled for this bucket")]
    BucketFeatureCentralizedShareableNotEnabled,
}



impl CentralizedShareLinkToken {
    pub fn new(token: ShareLinkTokenUnion, region: Option<DatacenterRegion>, bucket_features_flags: &BucketFeaturesFlags) -> Result<Self, CentralizedShareLinkTokenGeneratorError> {
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

