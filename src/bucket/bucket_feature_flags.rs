// All the available addons/features a bucket has active.
bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct BucketFeaturesFlags: u32 {
        /// Whether the bucket is searchable; true by default for all public buckets.
        const IS_SEARCHABLE                  = 0b0000_0000_0000_0001;
        /// Allows bucket files to be indexed for fast search.
        /// Refer to `IS_SEARCHABLE` for bucket-level indexing of name and bucket GUID.
        const IS_SEARCH_INDEXED              = 0b0000_0000_0000_0010;

        /// Requires additional plaintext (like a password) for bucket protection.
        const IS_PLAINTEXT_ACCESS_PROTECTED  = 0b0000_0000_0000_0100;

        /// Allows centralized sharing.
        const IS_CENTRALIZED_SHARABLE        = 0b0000_0000_0000_1000;
        /// Allows decentralized sharing.
        const IS_DECENTRALIZED_SHARABLE      = 0b0000_0000_0001_0000;

        /// Requires HTTPS for all uploads or downloads from the bucket.
        const IS_HTTPS_ONLY                  = 0b0000_0000_0010_0000;

        /// Indicates the bucket is pre-paid and has an `expire_at` field specifying the removal date.
        const IS_PRE_PAID                    = 0b0000_0000_0100_0000;

        /// Marks the bucket as containing NSFW content.
        const IS_NSFW                        = 0b0000_0000_1000_0000;

        /// Marks the bucket for data archiving.
        const SHOULD_ARCHIVE_DATA            = 0b0000_0001_0000_0000;
    }
}
