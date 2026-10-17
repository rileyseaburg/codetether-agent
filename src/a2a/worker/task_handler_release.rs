//! Adapt the worker runtime to shared task release handling.
use super::{WorkerTaskRuntime, task_timeline};
use anyhow::Result;
use codetether_a2a_http::worker::completion::{self, ReleaseTarget};
pub(super) async fn release_handled_task(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    outcome: super::task_outcome::TaskOutcome,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    completion::release_handled_task(
        ReleaseTarget {
            client: &runtime.client,
            server: &runtime.server,
            token: &runtime.token,
            worker_id: &runtime.worker_id,
        },
        task_id,
        outcome,
        timeline,
        &runtime.task_progress,
    )
    .await
}
