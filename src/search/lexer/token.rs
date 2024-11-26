use uuid::Uuid;

#[derive(Debug)]
pub enum SerachTerm {
    // either name of user or bucket.
    User(UserIdentifer),
    // either name of bucket or uuid of bucket.
    Bucket(BucketIdentifier),
    // Tag is a String only of one word.
    Tag(String),
    // description is a String of one or many words.
    Description(String),
}
#[derive(Debug)]
pub enum BucketIdentifier {
    Name(String), 
    Uuid(Uuid),
}

#[derive(Debug)]
pub enum UserIdentifer {
    Name(String), 
    Uuid(Uuid),
}