//! Worker event sink emit tool call if active.
use super::*;

/// Emits a `tool.call` event and reports whether a sink consumed it.
///
/// Callers use the return value to decide whether the legacy text-only callback
/// still needs to fire, so non-worker contexts keep their existing output.
/// # Arguments
/// * `tool_name` - Tool being invoked.
/// * `arguments` - Arguments to redact before emission.
/// # Returns
/// Whether a sink was observed before emitting the call.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit_tool_call_if_active;
/// let consumed = emit_tool_call_if_active("read", &serde_json::json!({}));
/// ```
pub fn emit_tool_call_if_active(tool_name: &str, arguments: &serde_json::Value) -> bool {
    if sink().is_none() {
        return false;
    }
    emit_tool_call(tool_name, arguments);
    true
}
