use core::slice::SlicePattern;
use std::fmt;
use serde::{Deserialize, Serialize};

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

impl SlicePattern for BucketGuid {
    type Item = u8;
    /// 256-bit array.
    fn as_slice(&self) -> &[Self::Item] {
        let mut slice = [0u8; 32];
        slice[0..16].copy_from_slice(self.user_id.as_bytes());
        slice[16..32].copy_from_slice(self.bucket_id.as_bytes());
        &slice
    }
}