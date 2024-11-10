use crate::bucket::bucket_features::BucketFeaturesFlags;
use crate::region::RegionCluster;
use rand::{CryptoRng, RngCore};
use url::Url;

/// A secrete share link, storing the token.
pub struct CentralizedSecretShareLinkToken {
    pub token: [u8; 32],
    pub region: Option<RegionCluster>,
}

pub enum

impl CentralizedSecretShareLinkToken {
    pub fn new(token: [u8; 32], region: Option<RegionCluster>) -> Self {
        Self {
            token,
            region,
        }
    }

    pub fn generate<TCSPRNG: RngCore + CryptoRng>(cspring :&mut TCSPRNG, region: Option<RegionCluster>, bucket_features_flags: &BucketFeaturesFlags) -> Self {
        if !bucket_features_flags.contains(BucketFeaturesFlags::IS_CENTRALIZED_SHARABLE) {

        }
        let mut token = [0u8; 32];
        cspring.fill_bytes(&mut token);
        Self {
            token,
            region,
        }
    }
}

pub struct CentralizedUrlEncodedSecreteShareLinkToken {
    pub url: Url,
}


impl TryInto<CentralizedUrlEncodedSecreteShareLinkToken> for CentralizedSecretShareLinkToken {
    type Error = ();

    fn try_into(self) -> Result<CentralizedUrlEncodedSecreteShareLinkToken, Self::Error> {
        todo!()
    }
}

