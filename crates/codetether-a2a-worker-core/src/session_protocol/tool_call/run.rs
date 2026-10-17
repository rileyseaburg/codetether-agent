//! Approval and execution order for one worker tool call.
use super::ToolCallHost;
use crate::{
    execution_policy::{AutoApprove, is_tool_allowed},
    session_protocol::ToolCall,
};
use codetether_tool_core::ToolResult;
use std::sync::Arc;
/// Emit, authorize, execute and record one provider tool call.
///
/// # Arguments
/// * `host` - Application session and execution effects.
/// * `auto_approve` - Worker tool approval policy.
/// * `cb` - Text fallback when no structured event sink is active.
/// * `tool_call` - Provider call ID, tool name and decoded arguments.
/// # Returns
/// Unit after recording the denial or completed tool result.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_a2a_worker_core::session_protocol::tool_call::ToolCallHost) {
/// use codetether_a2a_worker_core::{execution_policy::AutoApprove, session_protocol::tool_call::execute_tool_call};
/// execute_tool_call(host, AutoApprove::Safe, &None, ("id".into(), "read".into(), serde_json::json!({}))).await;
/// # }
/// ```

pub async fn execute_tool_call<H: ToolCallHost>(
    host: &mut H,
    auto_approve: AutoApprove,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    (tool_id, tool_name, tool_input): ToolCall,
) {
    // Publish the invocation with its arguments when a worker sink is active so
    // the transcript can render a real tool card; otherwise keep the historical
    // text marker for the plain output stream.
    if !crate::events::sink::emit_tool_call_if_active(&tool_name, &tool_input)
        && let Some(cb) = cb
    {
        cb(format!("[tool:start:{}]", tool_name));
    }
    if !is_tool_allowed(&tool_name, auto_approve) {
        host.record_result(
            tool_id,
            ToolResult::error(format!(
                "Tool '{}' requires approval but auto-approve policy is {:?}",
                tool_name, auto_approve
            )),
        );
        return;
    }
    let input = host.prepare_input(&tool_input);
    let output = host.run_tool(&tool_name, input).await;
    host.record_result(tool_id, output);
}
