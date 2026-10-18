//! Native standalone worker loop binding.
use super::{WorkerContext, reconnect_host::Native};
pub(super) async fn run_worker_loop(context: WorkerContext) -> anyhow::Result<()> {
    codetether_a2a_stream::worker_reconnect::run_worker_loop(&Native {
        context: &context,
        state: (),
    })
    .await
}
