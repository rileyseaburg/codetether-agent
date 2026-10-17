//! Structured worker event tool result event.
use super::*;

/// Builds a `tool.result` payload for one completed invocation.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::events::tool_result_event;
/// let event = tool_result_event("bash", true, "ok", None);
/// assert_eq!(event["payload"]["status"], serde_json::json!("ok"));
/// ```
/// # Arguments
/// * `tool_name` - Completed tool name.
/// * `success` - Whether execution succeeded.
/// * `output` - Output to bound by the event byte limit.
/// * `duration_ms` - Optional elapsed milliseconds.
/// # Returns
/// A typed result event with truncation and duration metadata.
pub fn tool_result_event(
    tool_name: &str,
    success: bool,
    output: &str,
    duration_ms: Option<u128>,
) -> Value {
    let (bounded, truncated) = clamp(output, MAX_OUTPUT_BYTES);
    let mut payload = serde_json::json!({
        "name": tool_name,
        "tool": tool_name,
        "status": if success { "ok" } else { "err" },
        "success": success,
        "output": bounded,
        "output_truncated": truncated,
        "output_bytes": output.len(),
    });
    if let Some(duration) = duration_ms
        && let Some(fields) = payload.as_object_mut()
    {
        fields.insert("duration_ms".into(), Value::from(duration as u64));
    }
    serde_json::json!({ "type": "tool.result", "payload": payload })
}
