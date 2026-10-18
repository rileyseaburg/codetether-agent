//! Native adapters for shared worker bootstrap sequencing.
use super::WorkerContext;
use anyhow::Result;
pub(super) async fn bootstrap_worker(context: &WorkerContext) -> Result<()> {
    codetether_native_worker::bootstrap::bootstrap_worker::<super::pending_tasks::Dispatcher>(
        context,
    )
    .await
}
pub(super) use codetether_native_worker::register_current_worker;
