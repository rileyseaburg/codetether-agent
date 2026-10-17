//! Worker event sink emit tool call.
use super::*;

/// Emits a `tool.call` event for one invocation.
/// # Arguments
/// * `tool_name` - Tool being invoked.
/// * `arguments` - Raw arguments redacted by the event constructor.
/// # Returns
/// Returns unit after attempting to emit the tool call.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit_tool_call;
/// emit_tool_call("read", &serde_json::json!({}));
/// ```
pub fn emit_tool_call(tool_name: &str, arguments: &serde_json::Value) {
    emit(
        format!("[tool:start:{tool_name}]"),
        super::super::tool_call_event(tool_name, arguments),
    );
}
