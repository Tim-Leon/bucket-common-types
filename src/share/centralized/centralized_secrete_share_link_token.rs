use crate::bucket::bucket_feature_flags::BucketFeaturesFlags;
use crate::region::RegionCluster;
use crate::share::share_link_token::SecreteShareLinkToken;

/// A secrete share link, storing the token.
pub struct CentralizedSecretShareLinkToken {
    pub token: SecreteShareLinkToken,
    pub region: Option<RegionCluster>,
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedSecreteShareLinkTokenGeneratorError {
    #[error("Centralized shareable feature is not enabled for this bucket")]
    BucketFeatureCentralizedShareableNotEnabled,
}

impl CentralizedSecretShareLinkToken {
    pub fn new(token: SecreteShareLinkToken, region: Option<RegionCluster>, bucket_features_flags: &BucketFeaturesFlags) -> Result<Self, CentralizedSecreteShareLinkTokenGeneratorError> {
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_CENTRALIZED_SHARABLE) {
            return Err(CentralizedSecreteShareLinkTokenGeneratorError::BucketFeatureCentralizedShareableNotEnabled)
        }
        Ok(Self {
            token,
            region,
        })
    }
}


