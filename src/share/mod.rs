#[cfg(feature = "secret_share_link")]
#[cfg(feature = "share_link")]
pub mod exclusive_share_link;
pub mod versioning;
pub mod centralized;
pub mod decentralized;
pub mod fully_qualified_domain_name;
mod share_link_token;

pub trait GenerateSyntaxDiagram {
    fn generate_syntax_diagram(&self) -> String;
}


/*
This module defines all the different share links that can be created.

Centralized based sharing uses a token based approach where the user request the server to create a share link. Note that the

The region is allways optional, if the user chose not to use it, a request from the client will have to be used in order to get where the bucket is stored, leading to additional indirection.
*/