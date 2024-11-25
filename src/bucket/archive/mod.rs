use crate::unix_timestamp::UnixTimestamp;

use super::{bucket_compression::BucketCompression, bucket_guid::BucketGuid, bucket_path::BucketRelativePath};


pub struct BucketMetadata {
    pub guid: BucketGuid,
    pub updated_at: UnixTimestamp,
    pub created_at: UnixTimestamp,
    pub capacity: u64,
    pub size: u64,
}

pub struct BucketObjectMetadata {
    pub updated_at: UnixTimestamp,
    pub created_at: UnixTimestamp,
    pub size: u64,
    pub encoding: ObjectEncoding,
    pub path: BucketRelativePath,
    pub hashes: ObjectHashes,
}


pub struct VirtualZipMetadate {
    pub compression_level: u8,
    pub compressed_parts: Vec<BucketRelativePath>,
    pub compression_algorithm: BucketCompression,
}





pub struct ObjectEncoding {
}


pub struct ObjectHashes {
    sha_256: Option<String>,
    sha_512: Option<String>,
    crc_32: Option<u32>,
    crc_64: Option<u64>,
    blake3: Option<String>,
}
