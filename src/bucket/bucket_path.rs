use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;
use std::str::pattern::Pattern;
use crate::bucket::bucket_guid::BucketGuid;

// Path implementation for bucket.

#[derive(Debug, Eq, PartialEq)]
pub struct BucketAbsolutePath {
    pub bucket_guid: BucketGuid,
    /// Relative path from BucketGuid, they are combined inorder to create absolute path.
    pub relative_path: BucketRelativePath,
}


impl BucketAbsolutePath {
    pub fn new(bucket_guid: BucketGuid, relative_path: BucketRelativePath) -> Self{
        BucketAbsolutePath {
            bucket_guid,
            relative_path,
        }
    }
}
/// The relative path from the bucket guid,
/// Every relative path starts with ``/``
/// Backslashes ``\` are not allowed.
/// Only alphanumeric and numbers are allowed and "-", "_"
/// Relative path can be combined with BucketGuid to create an BucketAbsolutePath.
#[derive(Debug, Eq, PartialEq)]
pub struct BucketRelativePath  {
    pub path: String,
}

impl FromStr for BucketRelativePath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    }
}
