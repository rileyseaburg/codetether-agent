//! Native adapters for shared worker bootstrap sequencing.
mod host;
use super::WorkerContext;
use anyhow::Result;
pub(super) async fn bootstrap_worker(context: &WorkerContext) -> Result<()> {
    codetether_a2a_worker_core::initialization::bootstrap::bootstrap_worker(&host::Native(context))
        .await
}
pub(super) async fn register_current_worker(context: &WorkerContext) -> Result<()> {
    codetether_a2a_http::worker::register_context(
        context,
        &context.args.token,
        context.args.public_url.as_deref(),
        super::provider_models::load_provider_models(),
    )
    .await
}
