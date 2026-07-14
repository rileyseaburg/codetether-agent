//! Deadline-bound invocation of a registered tool.

use super::state::State;
use serde_json::Value;
use std::time::Instant;
use tokio::time::timeout;

pub(super) use super::tool_raw_result::Result;

pub(super) async fn execute(state: &State, name: &str, args: Value) -> Result {
    let Some(tool) = state.registry.get(name) else {
        tracing::error!(tool = %name, "Unknown tool requested");
        return Result {
            output: format!("Unknown tool: {name}"),
            success: false,
            timed_out: false,
        };
    };
    let started = Instant::now();
    match timeout(
        state.deadline.saturating_duration_since(Instant::now()),
        tool.execute(args),
    )
    .await
    {
        Ok(Ok(result)) if result.success => {
            tracing::info!(tool = %name, duration_ms = started.elapsed().as_millis() as u64,
                "Tool execution completed");
            Result {
                output: result.output,
                success: true,
                timed_out: false,
            }
        }
        Ok(Ok(result)) => Result {
            output: format!("Tool error: {}", result.output),
            success: false,
            timed_out: false,
        },
        Ok(Err(error)) => Result {
            output: format!("Tool execution failed: {error}"),
            success: false,
            timed_out: false,
        },
        Err(_) => Result {
            output: format!("Tool execution timed out after {}s", state.timeout_secs),
            success: false,
            timed_out: true,
        },
    }
}
