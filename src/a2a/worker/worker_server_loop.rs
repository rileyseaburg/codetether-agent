//! Native task-dispatch selection for HTTP-integrated worker reconnection.
use super::{WorkerContext, pending_tasks::Dispatcher};
use crate::worker_server::WorkerServerState;
pub(super) async fn run_worker_server_loop(
    context: WorkerContext,
    server_state: WorkerServerState,
) -> anyhow::Result<()> {
    codetether_native_worker::reconnect::run_worker_server_loop::<Dispatcher>(context, server_state)
        .await
}
