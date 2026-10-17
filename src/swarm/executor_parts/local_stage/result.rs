//! Host metadata passed to shared agent-outcome conversion.
use super::{job::Job, retry::Outcome};
use crate::swarm::{SubTaskResult, SubTaskStatus};
use codetether_swarm_core::agent_result::{ExecutionSpec, from_agent};
use std::time::Duration;
pub(super) fn build(
    job: &Job,
    outcome: Outcome,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    let spec = ExecutionSpec {
        subtask_id: &job.id,
        instruction: &job.instruction,
        max_steps: job.max_steps,
        timeout_secs: job.timeout_secs,
    };
    from_agent(&spec, outcome, retries, elapsed)
}
