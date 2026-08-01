//! Tool execution with timeout handling.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub(super) async fn run_tool(
    registry: &crate::tool::ToolRegistry,
    tool_name: &str,
    input: serde_json::Value,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> String {
    let Some(tool) = registry.get(tool_name) else {
        return format!("Error: Unknown tool '{}'", tool_name);
    };
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(tool_name, &input).await
    {
        return blocked.output;
    }
    let timeout =
        super::super::super::env_u64("CODETETHER_WORKER_TOOL_TIMEOUT_SECS", 120).clamp(1, 3600);
    let started = Instant::now();
    match tokio::time::timeout(Duration::from_secs(timeout), tool.execute(input)).await {
        Ok(Ok(result)) => {
            // Emit the typed event when a worker sink is installed; otherwise
            // fall back to the historical text-only callback. Flattening the
            // tool name, status, and output into one string is what previously
            // made every transcript entry publish as `agent.message`.
            let emitted = super::super::super::session_event_sink::emit_tool_result_if_active(
                tool_name,
                result.success,
                &result.output,
                Some(started.elapsed().as_millis()),
            );
            if !emitted && let Some(cb) = cb {
                cb(format!("[tool:{}:{}] {}", tool_name, if result.success { "ok" } else { "err" }, crate::util::truncate_bytes_safe(&result.output, 500)));
            }
            result.output
        }
        Ok(Err(error)) => format!("Error: {}", error),
        Err(_) => {
            crate::tool::ToolResult::structured_error(
                "TOOL_TIMEOUT",
                tool_name,
                &format!("tool timed out after {}s", timeout),
                None,
                Some(serde_json::json!({"hint": "Narrow the request, set a more specific path/include filter, or retry with smaller scope."})),
            )
            .output
        }
    }
}
