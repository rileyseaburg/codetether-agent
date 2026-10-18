//! HTTP-server-integrated A2A worker entrypoint.

use anyhow::Result;

use crate::{cli::A2aArgs, worker_server::WorkerServerState};

use super::{bootstrap_worker, init_worker, run_worker_server_loop};

pub async fn run_with_state(args: A2aArgs, server_state: WorkerServerState) -> Result<()> {
    let context = init_worker(args).await?;
    server_state.attach_context(&context).await;
    bootstrap_worker(&context).await?;
    server_state.set_connected(false).await;
    run_worker_server_loop(context, server_state).await
}
