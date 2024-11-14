
/// Used by all url encoded share links both secrete and non-secrete.
/// Path depend on the version of the API.
#[derive(Debug, Clone, Eq, PartialEq, strum::EnumString, strum::Display)]
pub enum SharingApiPath {
    #[strum(serialize = "share/v1/")]
    V1 = 1, // First version number

    // Future versions can be added here, e.g., V2, V3
}


#[cfg(test)]
mod tests {
    use super::*; // Assuming PathWithVersion is in the same module

    // Test that the Display trait works and prints the correct string
    #[test]
    fn test_display_v1() {
        let version = SharingApiPath::V1;
        assert_eq!(version.to_string(), "share/v1/");
    }

    // Test that the enum can be parsed from a string (EnumString trait)
    #[test]
    fn test_enum_string() {
        let parsed: Result<SharingApiPath, _> = "share/v1/".parse();
        assert_eq!(parsed, Ok(SharingApiPath::V1));

        // Test invalid string that shouldn't parse to any variant
        let invalid: Result<SharingApiPath, _> = "share/v2/".parse();
        assert!(invalid.is_err());
    }

    // Test conversion between enum variants and their string representations
    #[test]
    fn test_variant_to_string() {
        let version = SharingApiPath::V1;
        assert_eq!(version.to_string(), "share/v1/");
    }

    // Test that the enum correctly parses to the expected variant
    #[test]
    fn test_from_str() {
        // Try parsing a valid string
        let parsed: SharingApiPath = "share/v1/".parse().unwrap();
        assert_eq!(parsed, SharingApiPath::V1);

        // Try parsing an invalid string
        let invalid_parse: Result<SharingApiPath, _> = "share/v2/".parse();
        assert!(invalid_parse.is_err());
    }
}
