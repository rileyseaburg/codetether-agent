//! JSON payloads for progress observations.
use super::*;

/// Build a `tool_heartbeat` observer payload.
///
/// # Arguments
/// Supply tool_call_id, name, elapsed_secs.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::tool_heartbeat;
/// let payload = tool_heartbeat("call-1", "read", 10);
/// assert_eq!(payload["type"], "tool_heartbeat");
/// ```
pub fn tool_heartbeat(tool_call_id: &str, name: &str, elapsed_secs: u64) -> Value {
    json!({ "type": "tool_heartbeat", "tool_call_id": tool_call_id, "name": name, "elapsed_secs": elapsed_secs })
}

/// Build a `tool_output_chunk` observer payload.
///
/// # Arguments
/// Supply tool_call_id, name, stream, chunk.
/// # Returns
/// The existing JSON event shape used by A2A observers.
/// # Examples
/// ```rust
/// use codetether_a2a_core::observer_payloads::tool_output_chunk;
/// let payload = tool_output_chunk("call-1", "read", "stdout", "text");
/// assert_eq!(payload["type"], "tool_output_chunk");
/// ```
pub fn tool_output_chunk(tool_call_id: &str, name: &str, stream: &str, chunk: &str) -> Value {
    json!({ "type": "tool_output_chunk", "tool_call_id": tool_call_id, "name": name, "stream": stream, "chunk": chunk })
}
