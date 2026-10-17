//! Structured worker event tool call event.
use super::*;

/// Builds a `tool.call` payload for one invocation.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::events::tool_call_event;
/// let event = tool_call_event("bash", &serde_json::json!({"command": "ls"}));
/// assert_eq!(event["type"], serde_json::json!("tool.call"));
/// ```
/// # Arguments
/// * `tool_name` - Tool being invoked.
/// * `arguments` - Raw tool arguments to redact before inclusion.
/// # Returns
/// A typed tool-call event with redacted arguments.
pub fn tool_call_event(tool_name: &str, arguments: &Value) -> Value {
    serde_json::json!({
        "type": "tool.call",
        "payload": {
            "name": tool_name,
            "tool": tool_name,
            "status": "started",
            "arguments": redact_arguments(arguments),
        },
    })
}
