//! Bind shared worker initialization to native application services.
mod host;
use super::WorkerContext;
use crate::cli::A2aArgs;
pub(super) async fn init_worker(args: A2aArgs) -> anyhow::Result<WorkerContext> {
    codetether_native_worker::initialization::initialize::<host::Native>(args).await
}
