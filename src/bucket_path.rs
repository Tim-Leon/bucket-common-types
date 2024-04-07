// Path implementation for bucket.

use crate::BucketGuid;

#[derive(Debug, Display, Eq)]
struct BucketPath {
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
