use super::Host;

/// Append tool output and images with explicit execution status for future recovery.
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
    let status = if result.success { "success" } else { "failure" };
    let mut message = codetether_tool_core::messages::raw_tool_result(tool_call_id, result);
    if let Some(codetether_provider_core::ContentPart::ToolResult { content, .. }) =
        message.content.first_mut()
    {
        // Keep original stdout whitespace and images; the standard renderer trims it.
        *content = format!("Tool call feedback\n- status: {status}\n\nOutput:\n{content}");
    }
    host.append(message);
}
