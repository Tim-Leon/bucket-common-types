use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use crate::util::DOMAIN_URL;

// Inspired https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html.
//strum::EnumString strum::Display
// TODO remove strum serialize?
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, Copy)]
pub enum BucketRegion {
    #[strum(serialize = "eu-central")]
    EuropeCentral,
    #[strum(serialize = "eu-north")]
    EuropeNorth,
    #[strum(serialize = "eu-south")]
    EuropeSouth,
    #[strum(serialize = "eu-west")]
    EuropeWest,
    #[strum(serialize = "eu-east")]
    EuropeEast,

    #[strum(serialize = "us-central")]
    AmericaCentral,
    #[strum(serialize = "us-north")]
    AmericaNorth,
    #[strum(serialize = "us-south")]
    AmericaSouth,
    #[strum(serialize = "us-west")]
    AmericaWest,
    #[strum(serialize = "us-east")]
    AmericaEast,

    #[strum(serialize = "af-central")]
    AfricaCentral,
    #[strum(serialize = "af-north")]
    AfricaNorth,
    #[strum(serialize = "af-south")]
    AfricaSouth,
    #[strum(serialize = "af-west")]
    AfricaWest,
    #[strum(serialize = "af-east")]
    AfricaEast,

    #[strum(serialize = "ap-central")]
    AsiaPacificCentral,
    #[strum(serialize = "ap-north")]
    AsiaPacificNorth,
    #[strum(serialize = "ap-south")]
    AsiaPacificSouth,
    #[strum(serialize = "ap-west")]
    AsiaPacificWest,
    #[strum(serialize = "ap-east")]
    AsiaPacificEast,

    #[strum(serialize = "me-central")]
    MiddleEastCentral,
    #[strum(serialize = "me-north")]
    MiddleEastNorth,
    #[strum(serialize = "me-south")]
    MiddleEastSouth,
    #[strum(serialize = "me-west")]
    MiddleEastWest,
    #[strum(serialize = "me-east")]
    MiddleEastEast,
    #[strum(serialize = "sa-central")]
    SouthAmericaCentral,
    #[strum(serialize = "sa-north")]
    SouthAmericaNorth,
    #[strum(serialize = "sa-south")]
    SouthAmericaSouth,
    #[strum(serialize = "sa-west")]
    SouthAmericaWest,
    #[strum(serialize = "sa-east")]
    SouthAmericaEast,
}


pub struct DatacenterRegion {
    region: BucketRegion,
    region_id: u32,
}

//eu-central-1
impl Display for DatacenterRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{}", self.region, self.region_id)
    }
}

impl FromStr for DatacenterRegion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.splitn(3,"-");
        todo!()
    }
}



pub type ClusterId = u32;


/// Contains region and cluster information.
/// Used in the subdomain to be used by DNS to resolve the ip address of that specific cluster.
/// BucketRegion field denoting the region.
/// And ClusterId referring to a specific cluster in the region.
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub struct RegionCluster {
    pub region: BucketRegion,
    pub cluster_id: ClusterId,
}

impl Display for RegionCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-{}", self.region, self.cluster_id)
    }
}

impl RegionCluster {
    pub fn to_url(&self) -> url::Url {
        url::Url::from_str(format!("{}.{}", self.to_string(), DOMAIN_URL).as_str()).unwrap()
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
    //#[error(transparent)]
    //FiledToParseBucketRegion(#[from] BucketRegionError),
}
/*
* Region is the location/zone of resource
* ClusterId is which one of the clusters inside of that Region. Users can be ensured that the interlinking between clusters id in the same region are at high speed.
* The region id and the cluster id are not the same.
* Example:
* eu-center-1-1
 */
impl FromStr for RegionCluster {
    type Err = RegionClusterParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let mut split = s.splitn(4,'-');
        //let region = split
        //    .next()
        //    .ok_or(RegionClusterParsingError::InvalidFormat)?;
        //let direction = split.next().unwrap();
        //let datacenter_id = split.next().unwrap();
        //let region_parsed = format!("{}-{}-{}", region, direction, datacenter_id).parse::<BucketRegion>()?;
        //let cluster_id = split
        //    .next()
        //    .ok_or(RegionClusterParsingError::InvalidFormat)?
        //    .parse()?;
        //Ok(RegionCluster {
        //    region: region_parsed,
        //    cluster_id,
        //})
        todo!()
    }
}

