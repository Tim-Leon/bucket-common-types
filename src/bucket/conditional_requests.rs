use core::range::Range;
use time::OffsetDateTime;

/// CAS
/// Compare and swap os usually the conditional part of a request that must be met inorder for  the request to be able to be completed.

pub enum BucketHash {
    Sha256([u8; 32]),
    Sha512([u8; 64]),
    None,
    // add more..
}


pub struct DataForRange {
    /// Byte range to compare against.
    range: Range<u32>,
    /// You are only able to do 1 Kbyte of compare and swap for data.
    data: Vec<u8>,
}




pub enum Condition {
    BucketMetadataCondition(BucketMetadataCondition),
    FileCondition(FileCondition),
}

pub enum BucketMetadataCondition {
    Hash(BucketHash), /// Will compare the hash to see if it matches.
    Tag(Vec<String>), /// every tag is an entity in a collection, you are able to check the tags for it.
    ModifyDate(OffsetDateTime), /// When check if it's the last date.
    Name(String),
    Size(u64),
}

pub enum FileCondition {
    Range(DataForRange),
    /// Compares the entire file hash. Maybe TODO: Remove???
    Data(BucketHash),

    Size(u64),
}

pub struct ConditionalRequest {

}