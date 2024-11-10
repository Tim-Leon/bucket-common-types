use base64::{engine::general_purpose, DecodeError, Engine};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use rand::{CryptoRng, RngCore};
use crate::bucket::bucket_features::BucketFeaturesFlags;
use crate::region::RegionCluster;
use crate::util::{DOMAIN_URL, SHARE_PATH_URL};


/*
*  Bucket share link
*  bucketdrive.co/api/v1/share/user_id/bucket_id#permissions#expires#signature
*/
#[derive(Debug)]
pub struct CentralizedShareLinkToken {
    pub token: [u8; 32],
    pub region: Option<RegionCluster>,
}

#[derive(thiserror::Error, Debug)]
pub enum CentralizedShareLinkTokenGeneratorError {
    #[error("Centralized shareable feature is not enabled for this bucket")]
    BucketFeatureCentralizedShareableNotEnabled,
}



impl CentralizedShareLinkToken {
    pub fn new(token: &[u8; 32], region: Option<RegionCluster>) -> CentralizedShareLinkToken {
        Self {
            token: *token,
            region,
        }
    }

    pub fn get_token(&self) -> [u8; 32] {
        self.token
    }

    pub fn generate<TCryptoRng: RngCore + CryptoRng>(
        cspring: &mut TCryptoRng,
        region: Option<RegionCluster>,
        bucket_features_flags: &BucketFeaturesFlags,
    ) -> Result<Self, CentralizedShareLinkTokenGeneratorError> {
        // Check if the bucket feature IS_CENTRALIZED_SHARABLE is enabled
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_CENTRALIZED_SHARABLE) {
            return Err(CentralizedShareLinkTokenGeneratorError::BucketFeatureCentralizedShareableNotEnabled);
        }

        // Generate a random token
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);

        Ok(Self {
            token,
            region,
        })
    }
}

