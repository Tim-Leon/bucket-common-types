use std::fmt::Debug;
use std::fmt::Display;

use crate::BucketGuid;

// Path implementation for bucket.

#[derive(Debug, Eq, PartialEq)]
pub struct BucketPath {
    pub bucket_guid: BucketGuid, 
    pub path: String,
}


impl BucketPath {
    pub fn new(bucket_guid: BucketGuid, path: String) -> Self{
        BucketPath {
            bucket_guid,
            path,
        }
    }
}
