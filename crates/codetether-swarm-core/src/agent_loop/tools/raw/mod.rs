//! Deadline-bound raw tool execution and result/image conversion.
//!
//! Pass a resolved tool to [execute]. [RawResult] preserves output, image parts,
//! success, and timeout state for subsequent event publication and rendering.
mod result;
use codetether_tool_core::Tool;
pub use result::RawResult;
use serde_json::Value;
use std::{sync::Arc, time::Instant};
use tokio::time::timeout;

/// Execute a resolved tool under the sub-agent's remaining deadline.
/// # Arguments
/// * `tool` — Resolved tool, or `None` for an unknown name.
/// * `name` — Tool name used by diagnostics.
/// * `args` — Prepared tool arguments.
/// * `deadline` — Absolute deadline for the agent.
/// * `timeout_secs` — Configured duration used in timeout messages.
/// # Returns
/// Output/images or the original unknown-tool, execution-error, or timeout result.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let result = codetether_swarm_core::agent_loop::tools::raw::execute(
///     None, "missing", serde_json::json!({}), std::time::Instant::now(), 30,
/// ).await;
/// assert!(!result.success);
/// # }
/// ```
pub async fn execute(
    tool: Option<Arc<dyn Tool>>,
    name: &str,
    args: Value,
    deadline: Instant,
    timeout_secs: u64,
) -> RawResult {
    let Some(tool) = tool else {
        tracing::error!(target: "codetether_agent::swarm::executor::agent_loop::tool_raw", tool = %name, "Unknown tool requested");
        return RawResult::failure(format!("Unknown tool: {name}"), false);
    };
    let started = Instant::now();
    match timeout(
        deadline.saturating_duration_since(Instant::now()),
        tool.execute(args),
    )
    .await
    {
        Ok(Ok(result)) => {
            tracing::info!(target: "codetether_agent::swarm::executor::agent_loop::tool_raw", tool = %name, success = result.success,
                duration_ms = started.elapsed().as_millis() as u64,
                output = %crate::agent_loop::events::preview(&result.output, 400),
                "Tool execution completed");
            RawResult::from_tool(result)
        }
        Ok(Err(error)) => RawResult::failure(format!("Tool execution failed: {error}"), false),
        Err(_) => RawResult::failure(
            format!("Tool execution timed out after {}s", timeout_secs),
            true,
        ),
    }
}
