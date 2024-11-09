use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum WebhookConnectionProtocol {
    HTTP,
    Websocket,
}

pub enum WebhookConnectionResolve {
    Domain(String),
    ConnectionId(String),
}