#[cfg(feature = "secret_share_link")]
#[cfg(feature = "share_link")]
pub mod exclusive_share_link;
pub mod versioning;
pub mod centralized;
pub mod decentralized;

pub trait GenerateSyntaxDiagram {
    fn generate_syntax_diagram(&self) -> String;
}


/*
This module defines all of the different share links that can be created.

Centralized based sharing uses a token based approach where the user request the server to create a share link. Note that the 
*/