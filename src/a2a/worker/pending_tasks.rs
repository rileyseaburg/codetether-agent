//! Native adapters for shared pending-task dispatch.
mod host;
use super::WorkerTaskRuntime;
pub(super) use host::Native as Dispatcher;
pub(super) async fn fetch_pending_tasks(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_native_worker::pending::fetch_pending_tasks::<Dispatcher>(runtime).await
}
pub(super) async fn poll_pending_tasks(runtime: &WorkerTaskRuntime) -> anyhow::Result<()> {
    codetether_native_worker::pending::poll_pending_tasks::<Dispatcher>(runtime).await
}
