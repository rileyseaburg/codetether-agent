//! Tool execution with timeout handling.

use std::{sync::Arc, time::Duration};

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
    match tokio::time::timeout(Duration::from_secs(timeout), tool.execute(input)).await {
        Ok(Ok(result)) => {
            if let Some(cb) = cb {
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
