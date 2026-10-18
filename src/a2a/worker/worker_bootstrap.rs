//! Native adapters for shared worker bootstrap sequencing.
mod host;
use super::{WorkerContext, register_worker};
use anyhow::Result;
pub(super) async fn bootstrap_worker(context: &WorkerContext) -> Result<()> {
    codetether_a2a_worker_core::initialization::bootstrap::bootstrap_worker(&host::Native(context))
        .await
}
pub(super) async fn register_current_worker(context: &WorkerContext) -> Result<()> {
    let codebases = context.shared_codebases.lock().await.clone();
    register_worker(
        &context.task_runtime.client,
        &context.server,
        &context.args.token,
        &context.worker_id,
        &context.name,
        &codebases,
        context.args.public_url.as_deref(),
    )
    .await
}
