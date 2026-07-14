//! Result mapping for one-shot ephemeral agent execution.

use crate::swarm::executor::AgentLoopExit;
use crate::tool::ToolResult;

pub(super) fn build(
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
    let deliverable_complete = crate::swarm::tool_policy::deliverable_error(&output).is_none();
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
