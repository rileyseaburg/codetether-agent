//! Native adapters for shared worker bootstrap sequencing.
mod host;
use super::WorkerContext;
use anyhow::Result;
pub(super) async fn bootstrap_worker(context: &WorkerContext) -> Result<()> {
    codetether_a2a_worker_core::initialization::bootstrap::bootstrap_worker(&host::Native(context))
        .await
}
pub(super) use codetether_native_worker::register_current_worker;
