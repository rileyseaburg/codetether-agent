//! Conversion of agent-loop outcomes into subtask results.

use super::{job::Job, retry::Outcome};
use crate::swarm::{SubTaskResult, SubTaskStatus};
use std::time::Duration;

pub(super) fn build(
    job: &Job,
    outcome: Outcome,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    match outcome {
        Ok((output, steps, tool_calls, exit)) => {
            super::result_success::build(job, output, steps, tool_calls, exit, retries, elapsed)
        }
        Err(error) => super::result_make::failed(job, error.to_string(), retries, elapsed),
    }
}
