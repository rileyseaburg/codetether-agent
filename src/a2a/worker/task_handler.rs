//! Application runtime adapter for the shared task lifecycle driver.
use super::{
    WorkerTaskRuntime, execute_claimed_task, sync_timeline_to_runtime, task_claim::claim_task,
    task_handler_release,
};
use anyhow::Result;
use codetether_a2a_worker_core::{
    claim::ClaimedTaskData,
    outcome::{TaskInner, TaskOutcome},
    task_driver::TaskHandlerHost,
    task_timeline::TaskTimeline,
};
use codetether_provenance::ClaimProvenance;
#[path = "task_handler_host.rs"]
mod host;
struct ApplicationHost<'a>(&'a WorkerTaskRuntime);
pub(super) async fn handle_task(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
) -> Result<()> {
    codetether_a2a_worker_core::task_driver::handle_task(&ApplicationHost(runtime), task).await
}
