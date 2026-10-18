use super::Host;

/// Append a raw tool response with the shared tool-result message encoding.
/// # Arguments
/// `host` owns the transcript; `tool_call_id` identifies the call; `result`
/// contains its output and status.
/// # Returns
/// Appends one message without performing an additional save.
/// # Examples
/// ```no_run
/// # fn record(host: &mut impl codetether_a2a_worker_core::session_protocol::transcript::Host, result: codetether_tool_core::ToolResult) {
/// codetether_a2a_worker_core::session_protocol::transcript::add_tool_result(host, "call-1".into(), result);
/// # }
/// ```
pub fn add_tool_result(
    host: &mut impl Host,
    tool_call_id: String,
    result: codetether_tool_core::ToolResult,
) {
    host.append(codetether_tool_core::messages::raw_tool_result(
        tool_call_id,
        result,
    ));
}
