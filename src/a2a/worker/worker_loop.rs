//! Native task-dispatch selection for standalone worker reconnection.
use super::{WorkerContext, pending_tasks::Dispatcher};
pub(super) async fn run_worker_loop(context: WorkerContext) -> anyhow::Result<()> {
    codetether_native_worker::reconnect::run_worker_loop::<Dispatcher>(context).await
}
