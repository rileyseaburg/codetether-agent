//! Request metadata diagnostics without request contents.
use serde_json::Value;

/// Log backend identity and request shape without message contents.
///
/// # Arguments
/// * `body` - Encoded request used only for lengths and tool presence.
/// * `backend` - Diagnostic backend label.
/// # Returns
/// Nothing; emits one structured tracing event.
pub fn log_http_responses_request(backend: &str, body: &Value) {
    let instructions_len = body
        .get("instructions")
        .and_then(Value::as_str)
        .map(str::len)
        .unwrap_or(0);
    let input_items = body
        .get("input")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    let has_tools = body
        .get("tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| !tools.is_empty());
    tracing::info!(
        backend,
        instructions_len,
        input_items,
        has_tools,
        "Sending HTTP responses request"
    );
}
