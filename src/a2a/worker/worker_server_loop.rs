//! Native HTTP-server-integrated worker loop binding.
use super::{WorkerContext, reconnect_host::Native};
use crate::worker_server::WorkerServerState;
pub(super) async fn run_worker_server_loop(
    context: WorkerContext,
    server_state: WorkerServerState,
) -> anyhow::Result<()> {
    codetether_a2a_stream::worker_reconnect::run_worker_server_loop(&Native {
        context: &context,
        state: &server_state,
    })
    .await
}
