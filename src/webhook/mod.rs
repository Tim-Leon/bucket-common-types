pub mod version;

use serde::{Deserialize, Serialize};
use url::Url;

pub struct WebhookConnectionId(pub String);

#[derive(Clone, Default, Eq, PartialEq, strum::Display, strum::EnumString)]
pub enum WebhookSignatureScheme {
    #[default]
    ED25519,
    HmacSha256,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhookConnectionProtocol {
    HTTP,
    Websocket,
}

/// When user creates a webhook the server needs some way for it to reconnect to the user.
/// Thi is either using a Connection id, which is temporary to the session, or Domain(url) which is a domain name owned by the user.
/// They can in such a way set the URL to redirect webhook events to their own domain name.
pub enum WebhookConnectionResolve {
    Domain(Url),
    ConnectionId(WebhookConnectionId),
}