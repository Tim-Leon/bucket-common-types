

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
    pub struct BucketPermissionFlags : u32 {
        // The ability to view the files that are in a bucket. basically just view the file-structure, with names, size and created_at and updated_at for every file.
        const VIEW =            0b00000000_00000000_00000000_00000001;
        /// The ability to read from the bucket, means the user is able to read from the files in the bucket.
        const READ =            0b00000000_00000000_00000000_00000010;
        /// The ability to write to the bucket, avoid.
        const WRITE =           0b00000000_00000000_00000000_00000100;
        /// The ability to delete file from the bucket.
        const DELETE_FILE =     0b00000000_00000000_00000000_00001000;
        /// The ability to delete the bucket
        const DELETE_BUCKET =   0b00000000_00000000_00000000_00010000;
        /// The ability to create ShareLinks or SecreteShareLinks of the bucket with the same level of permissions or subset, this is usually not preferred behaviour avoid.
        const SHARE_BUCKET =    0b00000000_00000000_00000000_00100000;
        /// The ability to clone the bucket, gives the user the ability to create their own copy of the bucket, to note unlike git there is no link between the clone and original.
        const CLONE =           0b00000000_00000000_00000000_01000000;
        /// The ability to search inside the bucket.
        const SEARCH =          0b00000000_00000000_00000000_10000000;
        /// The ability to expand the capacity of the bucket, avoid.
        const EXAPAND =         0b00000000_00000000_00000001_00000000;
        /// The ability to reduce the capacity of the bucket, avoid.
        const REDUCE =          0b00000000_00000000_00000010_00000000;
        /// Only accept registered users, permission denied is for every user without an account.
        const REGISTED_USER_ONLY = 0b00000000_00000000_00000100_00000000;
    }
}
