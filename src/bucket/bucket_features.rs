// All the available addons/features a bucket has active.
bitflags::bitflags! {
    #[derive(Debug,Copy, Clone, Eq,PartialEq)]
    pub struct BucketFeaturesFlags: u32 {
        /// Whether the bucket is searchable, will be true by default for all public buckets.
        const IS_SEARCHABLE         = 0b00000001;
        /// Adds additional plaintext for protection of the bucket. Normally plaintext in the form of a password.
        const IS_PLAINTEXT_ACCESS_PROTECTED = 0b00000010;
        /// Allow centralized sharing
        const IS_CENTRALIZED_SHARABLE           = 0b00000100;
        /// If the bucket files are indexed allowing for fast search. referee to ``IS_SEARCHABLE`` for bucket-level indexing of name and bucket_guid.
        const IS_SEARCH_INDEXED     = 0b00001000;
        /// The bucket is already paid for, there will be a expire_at field set for the bucket, of when the bucket will be removed
        const IS_PRE_PAID           = 0b00010000;
        /// When uploading or downloading from bucket it must be done over HTTPS.
        const IS_HTTPS_ONLY         = 0b00100000;
        /// The bucket is NSFW.
        const IS_NSFW               = 0b01000000;
        /// Allows decentralized sharing.
        const IS_DECETRALIZED_SHARABLE = 0b10000000;

        const SHOULD_ARCHIVE_DATA = 0b1000000;
    }
}
