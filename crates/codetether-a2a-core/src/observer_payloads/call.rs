//! JSON payloads for call observations.
use super::*;

/// Build a `tool_call_start` observer payload.
///
/// # Arguments
/// Supply tool_call_id, name, arguments.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::tool_call_start;
/// let payload = tool_call_start("call-1", "read", "{}");
/// assert_eq!(payload["type"], "tool_call_start");
/// ```
pub fn tool_call_start(tool_call_id: &str, name: &str, arguments: &str) -> Value {
    json!({ "type": "tool_call_start", "tool_call_id": tool_call_id, "name": name, "arguments": arguments })
}

/// Build a `tool_call_complete` observer payload. Output is limited to 500 Unicode characters.
///
/// # Arguments
/// Supply tool_call_id, name, output, success, duration_ms.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::tool_call_complete;
/// let payload = tool_call_complete("call-1", "read", "text", true, 20);
/// assert_eq!(payload["type"], "tool_call_complete");
/// ```
pub fn tool_call_complete(
    tool_call_id: &str,
    name: &str,
    output: &str,
    success: bool,
    duration_ms: u64,
) -> Value {
    json!({ "type": "tool_call_complete", "tool_call_id": tool_call_id, "name": name, "output": output.chars().take(500).collect::<String>(), "success": success, "duration_ms": duration_ms })
}
