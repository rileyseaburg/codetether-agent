use super::*;

/// Convert an agent attempt into a task result and lifecycle status.
///
/// # Arguments
/// * `spec` - Task identity, instruction and configured limits.
/// * `outcome` - Agent output/counts/exit or an execution error.
/// * `retries` - Number of retries already attempted.
/// * `elapsed` - Host-measured duration.
///
/// # Returns
/// The result record and status after output calibration and deliverable checks.
///
/// # Examples
/// ```
/// use codetether_swarm_core::{agent_result::{ExecutionSpec, from_agent}, agent_retry::AgentLoopExit};
/// let spec = ExecutionSpec { subtask_id: "task", instruction: "inspect", max_steps: 10, timeout_secs: 60 };
/// let (result, _) = from_agent(&spec, Ok(("done".into(), 1, 0, AgentLoopExit::Completed)), 0, std::time::Duration::ZERO);
/// assert!(result.success);
/// ```
pub fn from_agent(
    spec: &ExecutionSpec<'_>,
    outcome: AgentOutcome,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    match outcome {
        Ok((output, steps, tool_calls, exit)) => {
            super::success::build(spec, output, steps, tool_calls, exit, retries, elapsed)
        }
        Err(error) => super::make::failed(spec, error.to_string(), retries, elapsed),
    }
}
