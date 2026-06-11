pub(super) fn tool_call(step: usize, call_id: &str, tool_name: &str, arguments: &str) {
    tracing::info!(
        step,
        tool_call_id = %call_id,
        tool = %tool_name,
        "Executing tool"
    );
    tracing::debug!(
        tool = %tool_name,
        arguments,
        "Tool call arguments"
    );
}
