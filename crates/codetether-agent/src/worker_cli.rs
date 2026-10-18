//! Bind shared worker CLI coordination to native A2A entry points.
use codetether_agent::{a2a, cli::A2aArgs, worker_server::WorkerServerState};
struct Native;
impl codetether_worker_cli::Host for Native {
    async fn run(args: A2aArgs) -> anyhow::Result<()> {
        a2a::worker::run(args).await
    }
    async fn run_with_state(args: A2aArgs, state: WorkerServerState) -> anyhow::Result<()> {
        a2a::worker::run_with_state(args, state).await
    }
}
pub(super) async fn execute(args: A2aArgs) -> anyhow::Result<()> {
    codetether_worker_cli::execute::<Native>(args).await
}
