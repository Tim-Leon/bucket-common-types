pub const DOMAIN_URL: &str = "bucketdrive.co";
pub const DOMAIN_NAME: &str = "bucketdrive";
// Special filenames, don't use theses file names if you want to use default implementation of compression and client-side encryption.
// They are mad of 200 random numbers followed by appropriate extension.
pub const V1_FILENAME_SIGNATURE: &str =             "92788755736028022379305121440842586143418638236105851711197675179013382161072356419825278198723874874874859494628870810121940733961303339530173577799516328942537106587822423670792506928743842632331152.signature";
pub const V1_COMPRESSION_FILENAME_SIGNATURE: &str = "50117560752950974554535545014315583760674509623459165112648997632682653558635998762597818689412967765962736741537479767974824082115342533111123950955973630906771399675644385537645846754554702707298484.compression";

// Both secret-share-link and share-link use the same API endpoint for convenience
pub const SECRET_SHARE_PATH_URL: &str = "/api/v1/share";
pub const SHARE_PATH_URL: &str = "/api/v1/share";
