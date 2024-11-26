
// Only allows letters, numbers, and dashes
pub fn is_valid_bucket_name(name: &str) -> bool {
    if !name.is_ascii() {
        return false;
    }
    
    // Now we know it's ASCII-only, continue with other validations
    if name.is_empty() || name.len() > 63 {
        return false;
    }

    name.bytes().all(|b| {
        matches!(b,
            b'a'..=b'z' |
            b'A'..=b'Z' |
            b'0'..=b'9' |
            b'-'
        )
    })
}

/// Utf-8 and <= 1024 bytes
pub fn is_valid_bucket_description(description: &str) -> bool {
    description.len() <= 1024
}

pub fn is_valid_tag(tag: &str) -> bool {
    tag.len() <= 32
}

pub fn is_valid_user_name(name: &str) -> bool {
    name.len() <= 32
}
