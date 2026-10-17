//! blocking task operation.
use super::*;

/// `message/send` blocks by default.
pub fn blocking(params: &MessageSendParams) -> bool {
    params
        .configuration
        .as_ref()
        .and_then(|c| c.blocking)
        .unwrap_or(true)
}
