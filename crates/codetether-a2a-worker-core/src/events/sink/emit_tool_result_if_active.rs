//! Worker event sink emit tool result if active.
use super::*;

/// Emits a `tool.result` event and reports whether a sink consumed it.
/// # Arguments
/// * `tool_name` - Completed tool.
/// * `success` - Execution outcome.
/// * `output` - Tool result text.
/// * `duration_ms` - Optional execution duration.
/// # Returns
/// Whether a sink was observed before emitting the result.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit_tool_result_if_active;
/// let consumed = emit_tool_result_if_active("read", true, "done", None);
/// ```
pub fn emit_tool_result_if_active(
    tool_name: &str,
    success: bool,
    output: &str,
    duration_ms: Option<u128>,
) -> bool {
    if sink().is_none() {
        return false;
    }
    emit_tool_result(tool_name, success, output, duration_ms);
    true
}
