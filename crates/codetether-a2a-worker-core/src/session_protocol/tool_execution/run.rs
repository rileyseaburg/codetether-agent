//! Bounded tool execution after host policy approval.
use super::*;
/// Execute a known tool under runtime policy and a configured timeout.
///
/// # Arguments
/// * `host` - Application lookup and policy checks.
/// * `tool_name` - Requested tool identifier.
/// * `input` - Tool arguments.
/// * `cb` - Optional text output fallback when no structured sink is active.
/// # Returns
/// Tool output or the existing unknown-tool, policy, execution-error or timeout result.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &impl codetether_a2a_worker_core::session_protocol::tool_execution::ToolExecutionHost) {
/// use codetether_a2a_worker_core::session_protocol::tool_execution::run_tool;
/// let result = run_tool(host, "read", serde_json::json!({}), &None).await;
/// # }
/// ```

pub async fn run_tool<H: ToolExecutionHost>(
    host: &H,
    tool_name: &str,
    input: serde_json::Value,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> ToolResult {
    let Some(tool) = host.tool(tool_name) else {
        return ToolResult::error(format!("Error: Unknown tool '{tool_name}'"));
    };
    if let Some(blocked) = host.evaluate(tool_name, &input).await {
        return blocked;
    }
    let timeout = crate::env_u64("CODETETHER_WORKER_TOOL_TIMEOUT_SECS", 120).clamp(1, 3600);
    let started = Instant::now();
    match tokio::time::timeout(Duration::from_secs(timeout), tool.execute(input)).await {
        Ok(Ok(result)) => {
            report::success(tool_name, &result, started, cb);
            result
        }
        Ok(Err(error)) => ToolResult::error(format!("Error: {error}")),
        Err(_) => ToolResult::structured_error(
            "TOOL_TIMEOUT",
            tool_name,
            &format!("tool timed out after {}s", timeout),
            None,
            Some(
                serde_json::json!({"hint": "Narrow the request, set a more specific path/include filter, or retry with smaller scope."}),
            ),
        ),
    }
}
