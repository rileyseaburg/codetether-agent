//! Worker event sink emit tool result.
use super::*;

/// Emits a `tool.result` event, preserving the legacy text form.
///
/// The text keeps the historical `[tool:name:ok] …` shape so existing log
/// readers and the plain output stream are unaffected.
/// # Arguments
/// * `tool_name` - Completed tool.
/// * `success` - Execution outcome.
/// * `output` - Tool result text.
/// * `duration_ms` - Optional execution duration.
/// # Returns
/// Returns unit after attempting to emit the result.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit_tool_result;
/// emit_tool_result("read", true, "done", None);
/// ```
pub fn emit_tool_result(tool_name: &str, success: bool, output: &str, duration_ms: Option<u128>) {
    let status = if success { "ok" } else { "err" };
    let preview = codetether_util::truncate::truncate_bytes_safe(output, 500);
    emit(
        format!("[tool:{tool_name}:{status}] {preview}"),
        super::super::tool_result_event(tool_name, success, output, duration_ms),
    );
}
