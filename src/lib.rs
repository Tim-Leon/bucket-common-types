use std::{
    fmt::{self, write, Display},
    num::ParseIntError,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use strum::EnumIter;

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

// Inspired https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html.
//strum::EnumString strum::Display
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, Copy)]
pub enum BucketRegion {
    #[strum(serialize = "eu-central")]
    EuropeCentral(u32),
    #[strum(serialize = "eu-north")]
    EuropeNorth(u32),
    #[strum(serialize = "eu-south")]
    EuropeSouth(u32),
    #[strum(serialize = "eu-west")]
    EuropeWest(u32),
    #[strum(serialize = "eu-east")]
    EuropeEast(u32),

    #[strum(serialize = "us-central")]
    AmericaCentral(u32),
    #[strum(serialize = "us-north")]
    AmericaNorth(u32),
    #[strum(serialize = "us-south")]
    AmericaSouth(u32),
    #[strum(serialize = "us-west")]
    AmericaWest(u32),
    #[strum(serialize = "us-east")]
    AmericaEast(u32),

    #[strum(serialize = "af-central")]
    AfricaCentral(u32),
    #[strum(serialize = "af-north")]
    AfricaNorth(u32),
    #[strum(serialize = "af-south")]
    AfricaSouth(u32),
    #[strum(serialize = "af-west")]
    AfricaWest(u32),
    #[strum(serialize = "af-east")]
    AfricaEast(u32),

    #[strum(serialize = "ap-central")]
    AsiaPacificCentral(u32),
    #[strum(serialize = "ap-north")]
    AsiaPacificNorth(u32),
    #[strum(serialize = "ap-south")]
    AsiaPacificSouth(u32),
    #[strum(serialize = "ap-west")]
    AsiaPacificWest(u32),
    #[strum(serialize = "ap-east")]
    AsiaPacificEast(u32),

    #[strum(serialize = "me-central")]
    MiddleEastCentral(u32),
    #[strum(serialize = "me-north")]
    MiddleEastNorth(u32),
    #[strum(serialize = "me-south")]
    MiddleEastSouth(u32),
    #[strum(serialize = "me-west")]
    MiddleEastWest(u32),
    #[strum(serialize = "me-east")]
    MiddleEastEast(u32),

    #[strum(serialize = "sa-central")]
    SouthAmericaCentral(u32),
    #[strum(serialize = "sa-north")]
    SouthAmericaNorth(u32),
    #[strum(serialize = "sa-south")]
    SouthAmericaSouth(u32),
    #[strum(serialize = "sa-west")]
    SouthAmericaWest(u32),
    #[strum(serialize = "sa-east")]
    SouthAmericaEast(u32),
}
//eu-central-1
impl Display for BucketRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BucketRegion::EuropeCentral(x) => write!(f, "eu-central-{}", x),
            BucketRegion::EuropeNorth(x) => write!(f, "eu-north-{}", x),
            BucketRegion::EuropeSouth(x) => write!(f, "eu-south-{}", x),
            BucketRegion::EuropeWest(x) => write!(f, "eu-west-{}", x),
            BucketRegion::EuropeEast(x) => write!(f, "eu-east-{}", x),
            BucketRegion::AmericaCentral(x) => write!(f, "us-central-{}", x),
            BucketRegion::AmericaNorth(x) => write!(f, "us-north-{}", x),
            BucketRegion::AmericaSouth(x) => write!(f, "us-south-{}", x),
            BucketRegion::AmericaWest(x) => write!(f, "us-west-{}", x),
            BucketRegion::AmericaEast(x) => write!(f, "us-east-{}", x),
            BucketRegion::AfricaCentral(x) => write!(f, "af-central-{}", x),
            BucketRegion::AfricaNorth(x) => write!(f, "af-north-{}", x),
            BucketRegion::AfricaSouth(x) => write!(f, "af-south-{}", x),
            BucketRegion::AfricaWest(x) => write!(f, "af-west-{}", x),
            BucketRegion::AfricaEast(x) => write!(f, "af-east-{}", x),
            BucketRegion::AsiaPacificCentral(x) => write!(f, "ap-central-{}", x),
            BucketRegion::AsiaPacificNorth(x) => write!(f, "ap-north-{}", x),
            BucketRegion::AsiaPacificSouth(x) => write!(f, "ap-south-{}", x),
            BucketRegion::AsiaPacificWest(x) => write!(f, "ap-west-{}", x),
            BucketRegion::AsiaPacificEast(x) => write!(f, "ap-east-{}", x),
            BucketRegion::MiddleEastCentral(x) => write!(f, "me-central-{}", x),
            BucketRegion::MiddleEastNorth(x) => write!(f, "me-north-{}", x),
            BucketRegion::MiddleEastSouth(x) => write!(f, "me-south-{}", x),
            BucketRegion::MiddleEastWest(x) => write!(f, "me-west-{}", x),
            BucketRegion::MiddleEastEast(x) => write!(f, "me-east-{}", x),
            BucketRegion::SouthAmericaCentral(x) => write!(f, "sa-central-{}", x),
            BucketRegion::SouthAmericaNorth(x) => write!(f, "sa-north-{}", x),
            BucketRegion::SouthAmericaSouth(x) => write!(f, "sa-south-{}", x),
            BucketRegion::SouthAmericaWest(x) => write!(f, "sa-west-{}", x),
            BucketRegion::SouthAmericaEast(x) => write!(f, "sa-east-{}", x),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BucketRegionError {
    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid region: {0}")]
    InvalidRegion(String),

    #[error("Invalid number for {0} region")]
    InvalidNumber(String),

    #[error("Invalid variant for {0} region: {1}")]
    InvalidVariant(String, String),
    #[error("Empty direction")]
    EmptyDirection,
    #[error("Invalid region number")]
    InvalidRegionNumber,
}

impl FromStr for BucketRegion {
    type Err = BucketRegionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, '-');
        if let Some(region) = parts.next() {
            let direction = parts.next().ok_or(BucketRegionError::EmptyDirection)?;
            let region_num = parts
                .next()
                .and_then(|num| num.parse::<u32>().ok())
                .ok_or(BucketRegionError::InvalidRegionNumber)?;
            match region {
                "eu" => match direction {
                    "central" => Ok(BucketRegion::EuropeCentral(region_num)),
                    "north" => Ok(BucketRegion::EuropeNorth(region_num)),
                    "south" => Ok(BucketRegion::EuropeSouth(region_num)),
                    "west" => Ok(BucketRegion::EuropeWest(region_num)),
                    "east" => Ok(BucketRegion::EuropeEast(region_num)),
                    _ => Err(BucketRegionError::InvalidVariant(
                        "EU".to_string(),
                        s.to_string(),
                    )),
                },
                _ => Err(BucketRegionError::InvalidRegion(region.to_string())),
            }
        } else {
            Err(BucketRegionError::InvalidFormat)
        }
    }
}

pub type ClusterId = u32;

#[derive(PartialEq, Debug)]
pub struct RegionCluster {
    region: BucketRegion,
    cluster_id: ClusterId,
}

impl Display for RegionCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.region, self.cluster_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RegionClusterParsingError {
    #[error("Invalid region or cluster ID format")]
    InvalidFormat,
    #[error("Failed to parse cluster ID")]
    FailedToParseClusterId(#[from] ParseIntError),
    #[error("Invalid Region")]
    InvalidRegion(#[from] strum::ParseError),
    #[error(transparent)]
    FiledToParseBucketRegion(#[from] BucketRegionError),
}
/*
* Region is the location/zone of resource
* ClusterId is which one of the clusters inside of that Region. Users can be ensured that the interlinking between clusters id in the same region are at high speed.
* The region id and the cluster id are not the same.
* Example:
* eu-center-1:1
 */
impl FromStr for RegionCluster {
    type Err = RegionClusterParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let region = split
            .next()
            .ok_or(RegionClusterParsingError::InvalidFormat)?;
        let region_parsed = region.parse::<BucketRegion>()?;
        let cluster_id = split
            .next()
            .ok_or(RegionClusterParsingError::InvalidFormat)?
            .parse()?;
        Ok(RegionCluster {
            region: region_parsed,
            cluster_id,
        })
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum BucketCompression {
    ServerGzip,
    ServerBrotli,
    ServerZstd,

    ClientGzip,
    ClientBrotli,
    ClientZstd,

    ClientCustom(String),
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

/*
https://stripe.com/docs/products-prices/pricing-models#volume-tiers
User can only have one active subscription at a time.
either metered or subscription.
Both can not be active at the same time.
The user is able to do one time payments as well whenever.


The payment plans available will be

Pricing
- Pay Once.
- Metered. (Pay per usage)
- Subscription.
- One Time Payment.

When ever a user uses subscription or onetime-payment then user balance is used.
When a user runs out of balance they can no longer use services that cost.

metered subscription provide unlimited usage. But

*/
#[derive(
    Debug, Clone, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentModel {
    Metered,
    Subscription,
    OneTime,
}

/*
* The encryption has version contorll built in
* The format is Encryption:Version,
* None: uses no encryption.
* AES256: uses server side encryption.
* Zero-Knowledge: uses client side encryption.
* Custom: uses custom encryption. Relies on the client implementing the encryption specifics.
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BucketEncryption {
    None,
    AES256(u8),
    ZeroKnowledge(u8),
    // Must start with 'Custom-' and then the name of the encryption. with a max length of 64 characters entirely.
    Custom(String),
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum BucketEncryptionParsingError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("custom format too long")]
    CustomFormatTooLong,
    #[error("invalid delimiter")]
    InvalidDelimiter,
    #[error(transparent)]
    FaieldToParseVersion(#[from] ParseIntError),
}

impl FromStr for BucketEncryption {
    type Err = BucketEncryptionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let encryption = parts
            .next()
            .ok_or(BucketEncryptionParsingError::InvalidDelimiter)?;
        match encryption {
            "None" => Ok(BucketEncryption::None),
            "AES256" | "ZeroKnowledge" => {
                let version = u8::from_str(
                    parts
                        .next()
                        .ok_or(BucketEncryptionParsingError::InvalidDelimiter)?,
                )?;
                match encryption {
                    "AES256" => Ok(BucketEncryption::AES256(version)),
                    "ZeroKnowledge" => Ok(BucketEncryption::ZeroKnowledge(version)),
                    _ => unreachable!(), // Should not reach here due to match patterns
                }
            }
            x if x.starts_with("Custom-") => {
                if x.len() > 71 {
                    return Err(BucketEncryptionParsingError::CustomFormatTooLong);
                }
                Ok(BucketEncryption::Custom(s.to_string()))
            }
            _ => Err(BucketEncryptionParsingError::InvalidFormat),
        }
    }
}

impl Display for BucketEncryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BucketEncryption::None => write!(f, "None"),
            BucketEncryption::AES256(version) => write!(f, "AES256:{}", version),
            BucketEncryption::ZeroKnowledge(version) => write!(f, "ZeroKnowledge:{}", version),
            BucketEncryption::Custom(name) => write!(f, "Custom-{}", name),
        }
    }
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

/*
* Metered Subscription is the intended usage with monthly subscription being the main alternative in the form of. But to make it easier for regular users to use the service it also offers basic and premium plans.
*/
#[derive(
    Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentPlan {
    Free,
    //MonthlyBasic,
    //MonthlyPremium,
    MeteredSubscription,
    MonthlySubscription,
    OneTime,
    Canceled, // When using any subscription type and the user want's to cancel it. An update account with payment plan as canceled is requested.
}

/*
* https://stripe.com/en-se/guides/payment-methods-guide
*/
#[derive(
    Debug, Clone, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentMethod {
    Card,
    Wallet,
    BankDebit,
    //Crypto, // Support later, maybe?
}

bitflags::bitflags! {
    /// NOTE* can not just cast verifaction between u32 and i32 because of bit flip
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketGuid {
    pub user_id: uuid::Uuid,
    pub bucket_id: uuid::Uuid,
}

// Implements to string trait also.
impl Display for BucketGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(
            f,
            "{}{}",
            self.user_id.as_simple(),
            &self.bucket_id.simple().to_string()[..31] // Remove last 32nd character
        );
        // assert!(
        //     f.width().unwrap() <= 63, //Cehck if width() is correct usage.
        //     "Bucket name is too long and will cause issue with S3-API."
        // );
        res
    }
}

impl BucketGuid {
    pub fn new(user_id: uuid::Uuid, bucket_id: uuid::Uuid) -> Self {
        Self { user_id, bucket_id }
    }
}

#[cfg(test)]
mod bucket_encryption_tests {
    use super::*;
    #[test]
    fn test_validate_bucket_encryption() {
        // Test valid inputs
        assert_eq!(
            BucketEncryption::from_str("None"),
            Ok(BucketEncryption::None)
        );

        assert_eq!(
            BucketEncryption::from_str("AES256:1"),
            Ok(BucketEncryption::AES256(1))
        );
        


        assert_eq!(
            BucketEncryption::from_str("ZeroKnowledge:2"),
            Ok(BucketEncryption::ZeroKnowledge(2))
        );

        assert_eq!(
            BucketEncryption::from_str("Custom-MyEncryption"),
            Ok(BucketEncryption::Custom("Custom-MyEncryption".to_string()))
        );

        // Test invalid formats
        assert_eq!(
            BucketEncryption::from_str("InvalidEncryption"),
            Err(BucketEncryptionParsingError::InvalidFormat)
        );

        assert_eq!(
            BucketEncryption::from_str("AES256"), // Missing version
            Err(BucketEncryptionParsingError::InvalidDelimiter)
        );


    }
    #[test]
    fn test_valid_bucket_encryption_parsing() {
        assert_eq!(
            "None".parse::<BucketEncryption>(),
            Ok(BucketEncryption::None)
        );
        assert_eq!(
            "AES256:42".parse::<BucketEncryption>(),
            Ok(BucketEncryption::AES256(42))
        );
        assert_eq!(
            "ZeroKnowledge:5".parse::<BucketEncryption>(),
            Ok(BucketEncryption::ZeroKnowledge(5))
        );
        assert_eq!(
            "Custom-Test".parse::<BucketEncryption>(),
            Ok(BucketEncryption::Custom("Custom-Test".to_string()))
        );
    }

    #[test]
    fn test_invalid_bucket_encryption_parsing() {
        assert!("Invalid".parse::<BucketEncryption>().is_err());
        assert!("AES256:".parse::<BucketEncryption>().is_err()); // Missing version
        assert!(":42".parse::<BucketEncryption>().is_err()); // Missing encryption type
        assert!(
            "Custom-ThisIsAVeryLongStringThatShouldFailToParseWithOver64CharactersXXX"
                .parse::<BucketEncryption>()
                .is_err()
        ); // Too long custom encryption
    }

    #[test]
    fn test_invalid_version() {
        assert!("AES256:invalid".parse::<BucketEncryption>().is_err()); // Invalid version
    }

    #[test]
    fn test_custom_encryption_max_length() {
        let long_custom_encryption = format!("Custom-{}", "x".repeat(63)); // Create a custom encryption of max length

        assert_eq!(
            long_custom_encryption.parse::<BucketEncryption>(),
            Ok(BucketEncryption::Custom(long_custom_encryption))
        );

        let too_long_custom_encryption = format!("Custom-{}", "x".repeat(65)); // Exceeds max length
        assert!(too_long_custom_encryption
            .parse::<BucketEncryption>()
            .is_err());
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
        use super::*;

        #[test]
        fn test_bucket_region_parsing() {
            print!("{}", BucketRegion::from_str("eu-central-1").unwrap());
            assert_eq!(
                BucketRegion::from_str("eu-central-1").unwrap().to_string(),
                BucketRegion::EuropeCentral(1).to_string()
            );
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
