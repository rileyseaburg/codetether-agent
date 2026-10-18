//! HTTP-server-integrated A2A worker application binding.
use super::{pending_tasks::Dispatcher, worker_init::Hooks};
use crate::{cli::A2aArgs, worker_server::WorkerServerState};
use anyhow::Result;

pub async fn run_with_state(args: A2aArgs, server_state: WorkerServerState) -> Result<()> {
    codetether_native_worker::entrypoints::run_with_state::<Hooks, Dispatcher>(args, server_state)
        .await
}
