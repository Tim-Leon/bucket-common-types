use crate::bucket::bucket_guid::BucketGuid;
use std::fmt::Debug;
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

/// Must contain a file.
/// More restrictive than ``BucketRelativePath`` which only restrict the path to start with a forward slash.
#[derive(Debug, Eq, PartialEq)]
struct BucketRelativeAbsolutePath {
    pub path: String,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BucketRelativeAbsolutePathError {
    #[error("PathMustStartWithForwardSlash")]
    PathMustStartWithForwardSlash,
    #[error("Path contains invalid character at position {position}: '{invalid_char}'")]
    PathContainsInvalidCharacter { position: usize, invalid_char: char },
    #[error("Path mustn't be longer than {0}", BUCKET_RELATIVE_PATH_MAX_LENGTH)]
    RelativePathTooLong,
    #[error("Path does not contain file.something")]
    PathNotAbsolute,
}

impl FromStr for BucketRelativeAbsolutePath {
    type Err = BucketRelativeAbsolutePathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the path starts with a forward slash '/'
        if !s.starts_with('/') {
            return Err(BucketRelativeAbsolutePathError::PathMustStartWithForwardSlash);
        };
        // check path max length
        if s.len() > BUCKET_RELATIVE_PATH_MAX_LENGTH {
            return Err(BucketRelativeAbsolutePathError::RelativePathTooLong);
        }
        // Get the last segment
        let file_name = match s.rsplit_once('/') {
            Some((_, name)) if !name.is_empty() => name,
            _ => return Err(BucketRelativeAbsolutePathError::PathNotAbsolute),
        };

        // Check if there's a valid extension
        if !file_name.contains('.') || file_name.ends_with('.') {
            return Err(BucketRelativeAbsolutePathError::PathNotAbsolute);
        }
        // Check for invalid characters
        for (index, c) in s.chars().enumerate() {
            if !(c.is_alphanumeric() || c == '/' || c.is_numeric() || c == '-' || c == '_' || c == '.') {
                return Err(BucketRelativeAbsolutePathError::PathContainsInvalidCharacter {
                    position: index,
                    invalid_char: c,
                });
            }
        }

        Ok(Self {
            path: s.to_string(),
        })
    }
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


// ... existing code ...

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_bucket_relative_path_valid() {
        let valid_paths = vec![
            "/simple",
            "/path/to/something",
            "/with-dash",
            "/with_underscore",
            "/with123numbers",
        ];

        for path in valid_paths {
            assert!(BucketRelativePath::from_str(path).is_ok());
        }
    }

    #[test]
    fn test_bucket_relative_path_invalid() {
        let test_cases = vec![
            ("no_leading_slash", BucketRelativePathParserError::PathMustStartWithForwardSlash),
            ("/@invalid", BucketRelativePathParserError::PathContainsInvalidCharacter { position: 1, invalid_char: '@' }),
            ("/path/with space", BucketRelativePathParserError::PathContainsInvalidCharacter { position: 9, invalid_char: ' ' }),
        ];

        for (path, expected_err) in test_cases {
            let result = BucketRelativePath::from_str(path);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected_err);
        }
    }

    #[test]
    fn test_bucket_relative_absolute_path_valid() {
        let valid_paths = vec![
            "/file.txt",
            "/path/to/file.ext",
            "/document-123_test.pdf",
        ];

        for path in valid_paths {
            assert!(BucketRelativeAbsolutePath::from_str(path).is_ok());
        }
    }

    #[test]
    fn test_bucket_relative_absolute_path_invalid() {
        let test_cases = vec![
            ("no_leading_slash.txt", BucketRelativeAbsolutePathError::PathMustStartWithForwardSlash),
            ("/no_extension", BucketRelativeAbsolutePathError::PathNotAbsolute),
            ("/invalid.", BucketRelativeAbsolutePathError::PathNotAbsolute),
            ("/@invalid.txt", BucketRelativeAbsolutePathError::PathMustStartWithForwardSlash),
            ("/path/with space.txt", BucketRelativeAbsolutePathError::PathContainsInvalidCharacter { position: 9, invalid_char: ' ' }),
        ];

        for (path, expected_err) in test_cases {
            let result = BucketRelativeAbsolutePath::from_str(path);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected_err);
        }
    }

    #[test]
    fn test_path_length_limits() {
        // Create a path that exceeds the maximum length
        let long_path = format!("/{}", "a".repeat(BUCKET_RELATIVE_PATH_MAX_LENGTH));
        
        assert!(matches!(
            BucketRelativePath::from_str(&long_path),
            Err(BucketRelativePathParserError::RelativePathTooLong)
        ));

        assert!(matches!(
            BucketRelativeAbsolutePath::from_str(&format!("{}.txt", long_path)),
            Err(BucketRelativeAbsolutePathError::RelativePathTooLong)
        ));
    }

    #[test]
    fn test_bucket_absolute_path_creation() {
        let guid = BucketGuid {
            user_id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
        };
        let relative_path = BucketRelativePath::from_str("/test/path").unwrap();
        
        let absolute_path = BucketAbsolutePath::new(guid.clone(), relative_path);
        
        assert_eq!(absolute_path.bucket_guid, guid);
        assert_eq!(absolute_path.relative_path.path, "/test/path");
    }
    
}