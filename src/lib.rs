#![feature(slice_pattern)]
#![feature(slice_split_once)]
#![feature(associated_type_defaults)]
extern crate core;

use core::slice::SlicePattern;
use serde::de::Expected;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, LowerExp};
use std::{
    fmt::Display,
    str::FromStr,
};
use strum::IntoEnumIterator;


#[cfg(feature = "search_query")]
pub mod bucket_search_query;
#[cfg(feature = "unix_timestamp")]
pub mod unix_timestamp;
pub mod util;
pub mod encryption;
pub mod webhook;
pub mod region;
#[cfg(feature = "middleware")]
pub mod middleware;
#[cfg(feature = "key")]
pub mod key;
pub mod storage_engine;
pub mod bucket;
pub mod share;
mod user_settings;
pub mod token;
pub mod account;

#[derive(Clone, Default, Eq, PartialEq, strum::Display, strum::EnumString)]
pub enum WebhookSignatureScheme {
    #[default]
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
Debug, Clone, Default, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum BucketStorageClass {
    #[default]
    General,
    ReducedRedundancy,
}






pub struct BucketRedundancy {}









#[derive(
Debug, Clone, Default, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum DownloadFormat {
    #[default]
    Raw,
    Zip,
    Tar,
}






#[cfg(test)]
mod tests {
    use crate::bucket::bucket_guid::BucketGuid;
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

        //#[test]
        //fn test_bucket_region_parsing() {
        //    print!("{}", BucketRegion::from_str("eu-central-1").unwrap());
        //    assert_eq!(
        //        BucketRegion::from_str("eu-central-1").unwrap().to_string(),
        //        BucketRegion::EuropeCentral(1).to_string()
        //    );
        //}

        //#[test]
        //fn test_all_region_cluster_to_url() {
        //    let b = BucketRegion::AfricaCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::EuropeCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::EuropeNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::EuropeSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::EuropeWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::EuropeEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AmericaCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AmericaNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AmericaSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AmericaWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AmericaEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AfricaCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AfricaNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AfricaSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AfricaWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AfricaEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AsiaPacificCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AsiaPacificNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AsiaPacificSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AsiaPacificWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::AsiaPacificEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::MiddleEastCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::MiddleEastNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::MiddleEastSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::MiddleEastWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::MiddleEastEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::SouthAmericaCentral(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::SouthAmericaNorth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::SouthAmericaSouth(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::SouthAmericaWest(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //    let b = BucketRegion::SouthAmericaEast(0);
        //    let brc = RegionCluster {
        //        region: b,
        //        cluster_id: 1,
        //    };
        //    brc.to_url();
        //}
//
        //#[test]
        //fn test_region_cluster_valid_parsing() {
        //    assert_eq!(
        //        RegionCluster::from_str("eu-central-1:1").unwrap(),
        //        RegionCluster {
        //            region: BucketRegion::EuropeCentral(1),
        //            cluster_id: 1,
        //        }
        //    );
//
        //    // Add more valid cases...
        //}
//
        //#[test]
        //fn test_region_cluster_formatting() {
        //    let region_cluster = RegionCluster {
        //        region: BucketRegion::EuropeCentral(1),
        //        cluster_id: 1,
        //    };
        //    assert_eq!(region_cluster.to_string(), "eu-central-1:1");
        //}
    }
}
