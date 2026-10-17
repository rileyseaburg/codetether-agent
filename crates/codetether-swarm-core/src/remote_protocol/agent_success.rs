use super::*;
use crate::agent_retry::AgentLoopExit;
pub(super) fn build(
    payload: &RemoteSubtaskPayload,
    output: String,
    steps: usize,
    tool_calls: usize,
    exit_reason: AgentLoopExit,
    elapsed_ms: impl FnOnce() -> u64,
) -> SubTaskResult {
    let output = crate::deliverable::verify_output(&payload.instruction, &output);
    let (success, error) = match exit_reason {
        AgentLoopExit::Completed => match crate::deliverable::deliverable_error(&output) {
            Some(error) => (false, Some(error)),
            None => (true, None),
        },
        AgentLoopExit::MaxStepsReached => (
            false,
            Some(format!("Sub-agent hit max steps ({})", payload.max_steps)),
        ),
        AgentLoopExit::TimedOut => (
            false,
            Some(format!(
                "Sub-agent timed out after {}s",
                payload.timeout_secs
            )),
        ),
    };
    SubTaskResult {
        subtask_id: payload.subtask_id.clone(),
        subagent_id: format!("agent-{}", payload.subtask_id),
        success,
        result: output,
        steps,
        tool_calls,
        execution_time_ms: elapsed_ms(),
        error,
        artifacts: Vec::new(),

        retry_count: 0,
    }
}
