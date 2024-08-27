use std::{
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};
use std::env::var;
use std::fmt::{Debug, Formatter, LowerExp};
use serde::{Deserialize, Serialize, Serializer};
use serde::de::Expected;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use util::DOMAIN_URL;

#[cfg(feature = "search_query")]
pub mod bucket_search_query;
#[cfg(feature = "secret_share_link")]
#[cfg(feature = "share_link")]
pub mod exclusive_share_link;
#[cfg(feature = "secret_share_link")]
pub mod secret_share_link;
#[cfg(feature = "share_link")]
pub mod share_link;
#[cfg(feature = "unix_timestamp")]
pub mod unix_timestamp;
pub mod util;
pub mod bucket_path;
pub mod encryption;
pub mod webhook;
pub mod compression;
pub mod payment;
pub mod region;

#[derive(Clone, Eq, PartialEq, strum::Display, strum::EnumString)]
pub enum WebhookSignatureScheme {
    ED25519,
    HmacSha256,
}

/// Theses are all the supported encoding for files that are uploaded or downloaded.
#[derive(Clone, Eq, PartialEq, strum::Display, strum::EnumString)]
pub enum Encoding {
    LZ4,
    Zstd,
    Brotli,
    Deflate,
    Gzip,
    Custom(String),
}




/*
Video Codec Support Matrix TODO: Add...
*/
#[derive(
Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum VideoCodec {
    AV1,
    H264,
}

enum BucketPermission {}

#[derive(Debug, Clone, Eq, PartialEq)]
enum BucketAvailabilityStatus {
    Creating,
    Available,
    Deleting,
    Deleted,
    Updating,
    Archiving,
    Restoring,
    Unavailable,
    Unreachable,
    Corrupted,
}

#[derive(
Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum AvailabilityStatus {
    //TODO: REMOVE?
    Creating,
    Available,
    Deleting,
    Deleted,
    Updating,
    Archiving,
    Restoring,
    Unavailable,
    Unreachable,
    Corrupted,
}
/*
* General: Standard storage class. Will use HDD.
* Reduced Redundancy: Will use HDD but with less redundancy and more risk for the end user.
*/
#[derive(
Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum BucketStorageClass {
    General,
    ReducedRedundancy,
}

pub struct BucketRedundancy {}



#[derive(EnumString, PartialEq, Debug, Serialize, strum::Display, Clone, Eq, Deserialize)]
#[repr(u8)]
pub enum Role {
    #[strum(serialize = "S")]
    Server,
    #[strum(serialize = "C")]
    Client,
}

#[derive(
Debug, Clone, Copy, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum BucketVisibility {
    /// Anyone can see the bucket
    Public,
    /// Only author and invited users can see the bucket, Bucket will be made private-shared if private bucket is shared.
    PrivateShared,
    /// Only author.
    Private,
}

// All the available addons/features a bucket has active.
bitflags::bitflags! {
    #[derive(Debug,Copy, Clone, Eq,PartialEq)]
    pub struct BucketFeaturesFlags: u32 {
        const IS_SEARCHABLE         = 0b00000001;
        const IS_PASSWORD_PROTECTED = 0b00000010;
        const IS_SHARABLE           = 0b00000100;
        const IS_SEARCH_INDEXED     = 0b00001000;
    }
}

#[derive(
Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum DownloadFormat {
    Raw,
    Zip,
    Tar,
}



bitflags::bitflags! {
    /// NOTE* can not just cast verification between u32 and i32 because of bit flip
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Verification : i16 {
        const UNVERIFIED = 0b0000_0000_0000_0000;
        const EMAIL = 0b0000_0000_0000_0001;
        const PHONE = 0b0000_0000_0000_0010;
        const TOTP = 0b0000_0000_0000_0100;
    }
}

// BucketGuid is a combination between user_id and bucket_id.
// Max character length of 63 for aws s3 bucket name https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BucketGuid {
    pub user_id: uuid::Uuid,
    pub bucket_id: uuid::Uuid,
}

// Implements to string trait also.
impl fmt::Display for BucketGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(
            f,
            "{}{}",
            self.user_id.as_simple(),
            &self.bucket_id.simple().to_string()[..31] // Remove last 32nd character
        );
        debug_assert!(
            f.width().unwrap() <= 63, //Check if width() is correct usage.
            "Bucket name is too long and will cause issue with S3-API."
        );
        res
    }
}

impl BucketGuid {
    pub fn new(user_id: uuid::Uuid, bucket_id: uuid::Uuid) -> Self {
        Self { user_id, bucket_id }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_guid_display() {
        let bucket = BucketGuid {
            user_id: uuid::Uuid::new_v4(),
            bucket_id: uuid::Uuid::new_v4(),
        };
        let result = format!("{}", bucket); // Use Display directly
        assert!(result.len() <= 63, "Bucket name is too long for S3-API");
    }

    mod bucket_region {
        use crate::region::RegionCluster;
use crate::region::BucketRegion;
use super::*;

        //#[test]
        //fn test_bucket_region_parsing() {
        //    print!("{}", BucketRegion::from_str("eu-central-1").unwrap());
        //    assert_eq!(
        //        BucketRegion::from_str("eu-central-1").unwrap().to_string(),
        //        BucketRegion::EuropeCentral(1).to_string()
        //    );
        //}

        #[test]
        fn test_all_region_cluster_to_url() {
            let b = BucketRegion::AfricaCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::EuropeCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::EuropeNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::EuropeSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::EuropeWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::EuropeEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AmericaCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AmericaNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AmericaSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AmericaWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AmericaEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AfricaCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AfricaNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AfricaSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AfricaWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AfricaEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AsiaPacificCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AsiaPacificNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AsiaPacificSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AsiaPacificWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::AsiaPacificEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::MiddleEastCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::MiddleEastNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::MiddleEastSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::MiddleEastWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::MiddleEastEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::SouthAmericaCentral(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::SouthAmericaNorth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::SouthAmericaSouth(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::SouthAmericaWest(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
            let b = BucketRegion::SouthAmericaEast(0);
            let brc = RegionCluster {
                region: b,
                cluster_id: 1,
            };
            brc.to_url();
        }

        #[test]
        fn test_region_cluster_valid_parsing() {
            assert_eq!(
                RegionCluster::from_str("eu-central-1:1").unwrap(),
                RegionCluster {
                    region: BucketRegion::EuropeCentral(1),
                    cluster_id: 1,
                }
            );

            // Add more valid cases...
        }

        #[test]
        fn test_region_cluster_formatting() {
            let region_cluster = RegionCluster {
                region: BucketRegion::EuropeCentral(1),
                cluster_id: 1,
            };
            assert_eq!(region_cluster.to_string(), "eu-central-1:1");
        }
    }
}
