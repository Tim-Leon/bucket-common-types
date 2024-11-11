use serde::{Deserialize, Serialize};

/// Access-control for bucket.
#[derive(
    Debug, Clone, Default , Copy, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
)]
pub enum BucketVisibility {
    /// Anyone can see the bucket
    Public,
    /// Only author and invited users can see the bucket, Bucket will be made private-shared if private bucket is shared.
    PrivateShared,
    /// Only author.
    #[default]
    Private,
}