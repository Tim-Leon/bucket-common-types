use crate::bucket::bucket_features::BucketFeaturesFlags;
use crate::region::RegionCluster;
use rand::{CryptoRng, RngCore};
use url::Url;

/// A secrete share link, storing the token.
pub struct CentralizedSecretShareLinkToken {
    pub token: [u8; 32],
    pub region: Option<RegionCluster>,
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedSecreteShareLinkTokenGeneratorError {
    #[error("Centralized shareable feature is not enabled for this bucket")]
    BucketFeatureCentralizedShareableNotEnabled,
}

impl CentralizedSecretShareLinkToken {
    pub fn new(token: [u8; 32], region: Option<RegionCluster>) -> Self {
        Self {
            token,
            region,
        }
    }

    pub fn generate<TCSPRNG: RngCore + CryptoRng>(cspring :&mut TCSPRNG, region: Option<RegionCluster>, bucket_features_flags: &BucketFeaturesFlags) -> Result<Self, CentralizedSecreteShareLinkTokenGeneratorError> {
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_CENTRALIZED_SHARABLE) {
            return Err(CentralizedSecreteShareLinkTokenGeneratorError::BucketFeatureCentralizedShareableNotEnabled)
        }
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Ok(Self {
            token,
            region,
        })
    }
}


