//! Construction of local subtask result records.

use super::job::Job;
use crate::swarm::{SubTaskResult, SubTaskStatus};
use std::time::Duration;

pub(super) fn failed(
    job: &Job,
    error: String,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    (
        make(
            job,
            false,
            String::new(),
            0,
            0,
            Some(error),
            retries,
            elapsed,
        ),
        SubTaskStatus::Failed,
    )
}

pub(super) fn make(
    job: &Job,
    success: bool,
    output: String,
    steps: usize,
    tools: usize,
    error: Option<String>,
    retries: u32,
    elapsed: Duration,
) -> SubTaskResult {
    SubTaskResult {
        subtask_id: job.id.clone(),
        subagent_id: format!("agent-{}", job.id),
        success,
        result: output,
        steps,
        tool_calls: tools,
        execution_time_ms: elapsed.as_millis() as u64,
        error,
        artifacts: Vec::new(),
        retry_count: retries,
    }
}
