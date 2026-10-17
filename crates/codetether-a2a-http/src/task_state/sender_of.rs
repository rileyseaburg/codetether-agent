//! sender of task operation.
use super::*;

/// The peer's advertised name, when the client supplied one.
pub fn sender_of(params: &MessageSendParams) -> String {
    params
        .message
        .metadata
        .get("codetether.sender")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("remote-a2a")
        .to_string()
}
