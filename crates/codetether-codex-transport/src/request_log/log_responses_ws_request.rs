//! Request metadata diagnostics without request contents.
use serde_json::Value;

/// Log backend identity and request shape without message contents.
///
/// # Arguments
/// * `body` - Encoded request used only for lengths and tool presence.
/// * `backend` - Diagnostic backend label.
/// # Returns
/// Nothing; emits one structured tracing event.
pub fn log_responses_ws_request(body: &Value, backend: &str) {
    tracing::info!(
        backend,
        instructions_len = body
            .get("instructions")
            .and_then(|value| value.as_str())
            .map(str::len)
            .unwrap_or(0),
        input_items = body
            .get("input")
            .and_then(|value| value.as_array())
            .map(Vec::len)
            .unwrap_or(0),
        has_tools = body
            .get("tools")
            .and_then(|value| value.as_array())
            .is_some_and(|tools| !tools.is_empty()),
        "Sending responses websocket request"
    );
}
