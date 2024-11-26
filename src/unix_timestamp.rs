use prost_types::{Timestamp, TimestampError};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use time::{error::ComponentRange, OffsetDateTime};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct UnixTimestamp(pub OffsetDateTime);

impl UnixTimestamp {
    pub fn now_utc() -> Self {
        UnixTimestamp {
            0: OffsetDateTime::now_utc()
        }
    }
}

impl Default for UnixTimestamp {
    fn default() -> Self {
        UnixTimestamp(OffsetDateTime::UNIX_EPOCH)
    }
}

impl TryFrom<prost_types::Timestamp> for UnixTimestamp {
    type Error = ComponentRange;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        //sqlx::types::time::OffsetDateTime::
        let result = OffsetDateTime::from_unix_timestamp(value.seconds)?.replace_nanosecond(value.nanos as u32)?;
        Ok(UnixTimestamp(result))
    }
}
impl TryInto<prost_types::Timestamp> for UnixTimestamp {
    type Error = TimestampError;

    fn try_into(self) -> Result<prost_types::Timestamp, Self::Error> {
        Ok(Timestamp {
            seconds: self.0.unix_timestamp(),
            nanos: self.0.nanosecond() as i32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_default_unix_timestamp() {
        let default_unix_timestamp = UnixTimestamp::default();
        assert_eq!(default_unix_timestamp.0, OffsetDateTime::UNIX_EPOCH);
    }

    #[test]
    fn test_try_from_prost_timestamp_invalid() {
        // Create a prost Timestamp equivalent to some invalid time
        let prost_timestamp = Timestamp {
            seconds: -1, // Example of an invalid timestamp value
            nanos: 0,
        };

        let result = UnixTimestamp::try_from(prost_timestamp);
        assert!(result.is_err());

        if let Err(_error) = result {
            // Ensure the correct error type is returned
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_from_into_conversions() {
        let time: UnixTimestamp = UnixTimestamp::now_utc();
        let prost: Timestamp = time.try_into().unwrap();
        let time2 = UnixTimestamp::try_from(prost).unwrap();
        assert_eq!(time, time2)
    }
}
