use super::*;
use crate::agent_retry::AgentOutcome;
/// Construct a remote task result using its execution outcome and deferred timing.
///
/// # Arguments
/// * `payload` - Remote task identity, instruction and limits.
/// * `outcome` - Agent output or execution error.
/// * `elapsed_ms` - Clock read deferred until the result's timing field is built.
///
/// # Returns
/// A remote result with the existing output policy and zero retry count.
///
/// # Examples
/// ```no_run
/// # fn example(payload: &codetether_swarm_core::remote_protocol::RemoteSubtaskPayload) {
/// let result = codetether_swarm_core::remote_protocol::from_agent_result(
///     payload, Err(anyhow::anyhow!("failed")), || 10,
/// );
/// assert_eq!(result.execution_time_ms, 10);
/// # }
/// ```
pub fn from_agent_result(
    payload: &RemoteSubtaskPayload,
    outcome: AgentOutcome,
    elapsed_ms: impl FnOnce() -> u64,
) -> SubTaskResult {
    match outcome {
        Ok((output, steps, tool_calls, exit_reason)) => {
            super::agent_success::build(payload, output, steps, tool_calls, exit_reason, elapsed_ms)
        }
        Err(error) => SubTaskResult {
            subtask_id: payload.subtask_id.clone(),
            subagent_id: format!("agent-{}", payload.subtask_id),
            success: false,
            result: String::new(),
            steps: 0,
            tool_calls: 0,
            execution_time_ms: elapsed_ms(),
            error: Some(error.to_string()),
            artifacts: Vec::new(),

            retry_count: 0,
        },
    }
}
