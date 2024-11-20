use crate::bucket::bucket_guid::BucketGuid;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

// Path implementation for bucket.

#[derive(Debug, Eq, PartialEq)]
pub struct BucketAbsolutePath {
    pub bucket_guid: BucketGuid,
    /// Relative path from BucketGuid, they are combined inorder to create absolute path.
    pub relative_path: BucketRelativePath,
}


impl BucketAbsolutePath {
    pub fn new(bucket_guid: BucketGuid, relative_path: BucketRelativePath) -> Self{
        BucketAbsolutePath {
            bucket_guid,
            relative_path,
        }
    }
}

pub struct BucketCommonPrefixedPath {
    prefix: BucketAbsolutePath, 
    paths: Vec<BucketRelativePath>,
}

/// Must contain a file
struct BucketRelativeAbsolutePath {
    
}

/// The relative path from the bucket guid,
/// Every relative path starts with ``/``
/// Only alphanumeric and numbers are allowed and "-", "_"
/// Relative path can be combined with BucketGuid to create an BucketAbsolutePath.
#[derive(Debug, Eq, PartialEq)]
pub struct BucketRelativePath  {
    pub path: String,
}

pub const BUCKET_RELATIVE_PATH_MAX_LENGTH: usize = 1024 - BucketGuid::size();

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum BucketRelativePathParserError {
    #[error("Path must start with a forward slash '/'")]
    PathMustStartWithForwardSlash,

    #[error("Path contains invalid character at position {position}: '{invalid_char}'")]
    PathContainsInvalidCharacter { position: usize, invalid_char: char },
    #[error("Path mustn't be longer than {0}", BUCKET_RELATIVE_PATH_MAX_LENGTH)]
    RelativePathTooLong,
}

impl FromStr for BucketRelativePath {
    type Err = BucketRelativePathParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the path starts with a forward slash '/'
        if !s.starts_with('/') {
            return Err(BucketRelativePathParserError::PathMustStartWithForwardSlash);
        }

        // check path max length
        if s.len() > BUCKET_RELATIVE_PATH_MAX_LENGTH {
            return Err(BucketRelativePathParserError::RelativePathTooLong);
        }
        let segments : u32 = 0;
        // Check for invalid characters
        for (index, c) in s.chars().enumerate() {
            if !(c.is_alphanumeric() || c == '/' || c.is_numeric() || c == '-' || c == '_') {
                return Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                    position: index,
                    invalid_char: c,
                });
            }
        }

        Ok(BucketRelativePath {
            path: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_valid_paths() {
        // Test a basic valid path
        let path = BucketRelativePath::from_str("/valid_path");
        assert_eq!(path, Ok(BucketRelativePath { path: "/valid_path".to_string() }));

        // Test a path with numbers and allowed characters
        let path = BucketRelativePath::from_str("/path123_with-allowed_chars");
        assert_eq!(path, Ok(BucketRelativePath { path: "/path123_with-allowed_chars".to_string() }));

        // Test a simple path with only a forward slash
        let path = BucketRelativePath::from_str("/");
        assert_eq!(path, Ok(BucketRelativePath { path: "/".to_string() }));
    }

    #[test]
    fn test_path_without_leading_slash() {
        // Path without leading slash should return an error
        let path = BucketRelativePath::from_str("no_slash");
        assert_eq!(path, Err(BucketRelativePathParserError::PathMustStartWithForwardSlash));
    }

    #[test]
    fn test_path_with_invalid_characters() {
        // Path with invalid characters should return an error
        let path = BucketRelativePath::from_str("/invalid!@#$%");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 8,
                invalid_char: '!'
            })
        );

        // Path containing spaces
        let path = BucketRelativePath::from_str("/invalid path");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 8,
                invalid_char: ' '
            })
        );

        // Path with a backslash
        let path = BucketRelativePath::from_str("/invalid\\path");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 8,
                invalid_char: '\\'
            })
        );

        // Path with restricted patterns like ".." or "."
        let path = BucketRelativePath::from_str("/invalid/..");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 9,
                invalid_char: '.'
            })
        );

        let path = BucketRelativePath::from_str("/invalid/.");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 9,
                invalid_char: '.'
            })
        );

        // Path with colon
        let path = BucketRelativePath::from_str("/invalid:path");
        assert_eq!(
            path,
            Err(BucketRelativePathParserError::PathContainsInvalidCharacter {
                position: 8,
                invalid_char: ':'
            })
        );
    }

    #[test]
    fn test_path_with_valid_special_characters() {
        // Test that dashes and underscores are allowed in paths
        let path = BucketRelativePath::from_str("/valid_path-123_ok");
        assert_eq!(path, Ok(BucketRelativePath { path: "/valid_path-123_ok".to_string() }));

        // Test that numbers, dashes, and underscores work together
        let path = BucketRelativePath::from_str("/path123_with-numbers_and_letters");
        assert_eq!(path, Ok(BucketRelativePath { path: "/path123_with-numbers_and_letters".to_string() }));
    }

    #[test]
    fn test_path_too_long() {
        // Generate a path exceeding the max length
        let long_path = format!("/{}", "a".repeat(BUCKET_RELATIVE_PATH_MAX_LENGTH));
        let too_long_path = format!("/{}", "a".repeat(BUCKET_RELATIVE_PATH_MAX_LENGTH + 1));

        // Valid path within max length
        let path = BucketRelativePath::from_str(&long_path);
        assert_eq!(path, Ok(BucketRelativePath { path: long_path }));

        // Path exceeding max length should return an error
        let path = BucketRelativePath::from_str(&too_long_path);
        assert_eq!(path, Err(BucketRelativePathParserError::RelativePathTooLong));
    }
}
