use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};

/// 3 Characters at a maximum
const REGION_ID_MAX_LENGTH: usize = 3;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Copy, strum::Display, Hash)]
pub enum Region {
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DatacenterRegion {
    region: Region,
    id: Box<str>,
}

// Implementing Display trait for DatacenterRegion
impl Display for DatacenterRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.region, self.id)
    }
}

impl FromStr for DatacenterRegion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        if parts.len() != 2 {
            return Err(());
        }

        // Parse the region string to the corresponding enum variant
        let region_str = parts[0];
        let region = region_str.parse().map_err(|_| ())?;

        // Check the ID length and convert to Box<str>
        let id = parts[1];
        if id.len() > REGION_ID_MAX_LENGTH {
            return Err(());
        }

        Ok(DatacenterRegion {
            region,
            id: id.into(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test for valid region and ID string parsing
    #[test]
    fn test_valid_datacenter_region() {
        let input = "eu-central-001";
        let result = DatacenterRegion::from_str(input);
        assert!(result.is_ok());
        let datacenter_region = result.unwrap();
        assert_eq!(datacenter_region.region, Region::EuropeCentral);
        assert_eq!(datacenter_region.id, "001".into());
    }

    // Test for invalid region string (non-existent region)
    #[test]
    fn test_invalid_region() {
        let input = "invalid-region-001";
        let result = DatacenterRegion::from_str(input);
        assert!(result.is_err());
    }

    // Test for ID length exceeding the maximum allowed
    #[test]
    fn test_id_too_long() {
        let input = "eu-central-1234";
        let result = DatacenterRegion::from_str(input);
        assert!(result.is_err());
    }

    // Test for valid region string and invalid format (missing ID)
    #[test]
    fn test_missing_id() {
        let input = "eu-central";
        let result = DatacenterRegion::from_str(input);
        assert!(result.is_err());
    }

    // Test for Display trait
    #[test]
    fn test_display_trait() {
        let datacenter_region = DatacenterRegion {
            region: Region::EuropeCentral,
            id: "001".into(),
        };
        assert_eq!(datacenter_region.to_string(), "eu-central-001");
    }
}
