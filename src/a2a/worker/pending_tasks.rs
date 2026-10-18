//! Native adapters for shared pending-task dispatch.
mod host;
use super::WorkerTaskRuntime;
pub(super) async fn fetch_pending_tasks(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_a2a_worker_core::dispatch::pending::fetch_pending_tasks(&host::Native(runtime)).await
}
pub(super) async fn poll_pending_tasks(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_a2a_worker_core::dispatch::pending::poll_pending_tasks(&host::Native(runtime)).await
}
