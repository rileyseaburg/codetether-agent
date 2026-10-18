//! Native adapter for shared claimed-task execution.
mod host;
use super::{WorkerTaskRuntime, task_timeline};
use crate::provenance::ClaimProvenance;
use anyhow::Result;
/// Run a claimed task end to end and return status, result, error, session ID.
pub(super) async fn execute_claimed_task<'a>(
    runtime: &WorkerTaskRuntime,
    task: &'a serde_json::Value,
    task_id: &'a str,
    title: &'a str,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<(&'static str, Option<String>, Option<String>, Option<String>)> {
    codetether_a2a_worker_core::task_driver::claimed::execute(
        &host::Native(runtime),
        task,
        task_id,
        title,
        claim_provenance,
        provider_keys,
        timeline,
    )
    .await
}
