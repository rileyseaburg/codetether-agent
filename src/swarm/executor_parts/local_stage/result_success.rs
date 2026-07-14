//! Successful and safety-limited local agent outcomes.

use super::super::AgentLoopExit;
use super::{job::Job, result_make};
use crate::swarm::{SubTaskResult, SubTaskStatus, tool_policy};
use std::time::Duration;

pub(super) fn build(
    job: &Job,
    output: String,
    steps: usize,
    tools: usize,
    exit: AgentLoopExit,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    let output = tool_policy::verify_output(&job.instruction, &output);
    let error = match exit {
        AgentLoopExit::Completed => tool_policy::deliverable_error(&output),
        AgentLoopExit::MaxStepsReached => {
            Some(format!("Sub-agent hit max steps ({})", job.max_steps))
        }
        AgentLoopExit::TimedOut => Some(format!("Sub-agent timed out after {}s", job.timeout_secs)),
    };
    let success = error.is_none();
    let status = match exit {
        AgentLoopExit::Completed if success => SubTaskStatus::Completed,
        AgentLoopExit::TimedOut => SubTaskStatus::TimedOut,
        AgentLoopExit::Completed | AgentLoopExit::MaxStepsReached => SubTaskStatus::Failed,
    };
    (
        result_make::make(job, success, output, steps, tools, error, retries, elapsed),
        status,
    )
}
