/// Flags that define the behavior of storage operations.
/// These flags apply to both bucket-level and file-level operations,
/// specifying how to handle operations with potentially destructive effects.
bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct StorageOperationBehaviorFlags: u8 {
        /// Whether to allow partial operations if they cannot be fully completed.
        const ALLOW_PARTIAL             = 0b0000_0001;

        /// Allow operations to overwrite existing data.
        const SHOULD_OVERWRITE          = 0b0000_0010;

        /// Indicates that the operation can be destructive to storage capacity of the bucket.
        const IS_CAPACITY_DESTRUCTIVE   = 0b0000_0100;

    }
}
