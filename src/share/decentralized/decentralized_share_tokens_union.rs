use super::{decentralized_secrete_share_token::DecentralizedSecretShareToken, decentralized_share_token::DecentralizedShareToken};



// Just an enum used to store share link.
pub enum DecentralizedSecretShareTokenUnion {
    DecentralizedShareToken(DecentralizedShareToken),
    DecentralizedSecretShareToken(DecentralizedSecretShareToken),
}
