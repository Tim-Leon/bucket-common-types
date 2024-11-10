
/// Used by all url encoded share links both secrete and non-secrete.
#[derive(Clone, Eq, PartialEq)]
pub enum UrlEncodedShareLinksVersioning {
    V1 = 1, // First version
    // Future versions can be added here, e.g., V2, V3
}