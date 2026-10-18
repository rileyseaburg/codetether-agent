//! Bind shared worker initialization to native application services.
mod host;
use super::WorkerContext;
use crate::cli::A2aArgs;
pub(super) async fn init_worker(args: A2aArgs) -> anyhow::Result<WorkerContext> {
    codetether_a2a_worker_core::initialization::initialize::init_worker::<host::Native>(args).await
}
