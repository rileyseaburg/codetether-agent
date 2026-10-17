//! Result mapping for one-shot ephemeral agent execution.

use codetether_swarm_core::agent_retry::AgentLoopExit;
use codetether_tool_core::ToolResult;

/// Format the result of an existing one-shot child execution.
/// # Arguments
/// * `name` — Ephemeral child display name.
/// * `outcome` — Output, step/tool counts and loop exit, or execution error.
/// * `warning` — Optional model-cost warning.
/// # Returns
/// The unchanged JSON result or execution error; only completed deliverables succeed.
/// # Examples
/// ```rust
/// use codetether_agent_spawn::ephemeral_result;
/// use codetether_swarm_core::agent_retry::AgentLoopExit;
/// let result = ephemeral_result("once", Ok(("done".into(), 1, 0, AgentLoopExit::Completed)), None);
/// assert!(result.success);
/// ```
pub fn build(
    name: &str,
    outcome: anyhow::Result<(String, usize, usize, AgentLoopExit)>,
    warning: Option<&str>,
) -> ToolResult {
    let (output, steps, tool_calls, exit) = match outcome {
        Ok(value) => value,
        Err(error) => {
            return ToolResult::error(format!(
                "Ephemeral agent @{name} failed to execute: {error}"
            ));
        }
    };
    let deliverable_complete =
        codetether_swarm_core::deliverable::deliverable_error(&output).is_none();
    let payload = serde_json::json!({
        "agent": name,
        "ephemeral": true,
        "persisted": false,
        "output": output,
        "steps": steps,
        "tool_calls": tool_calls,
        "warning": warning,
        "exit": format!("{exit:?}"),
    });
    let text = payload.to_string();
    match exit {
        AgentLoopExit::Completed if deliverable_complete => ToolResult::success(text),
        AgentLoopExit::Completed => ToolResult::error(text),
        AgentLoopExit::MaxStepsReached | AgentLoopExit::TimedOut => ToolResult::error(text),
    }
}

#[cfg(test)]
#[path = "ephemeral_result_tests.rs"]
mod tests;
