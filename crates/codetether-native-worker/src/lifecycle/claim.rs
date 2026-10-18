//! Adapt worker state to shared task claiming.
use crate::{WorkerTaskRuntime, task_timeline};
use anyhow::Result;
use codetether_a2a_http::worker::claim::ClaimedTaskData;
pub(super) async fn claim_task(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<ClaimedTaskData>> {
    codetether_a2a_http::worker::claim::claim_task(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.worker_id,
        &runtime.agent_name,
        task_id,
        timeline,
    )
    .await
}
