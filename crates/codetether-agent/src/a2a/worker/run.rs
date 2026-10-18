//! Standalone A2A worker application binding.
use super::{pending_tasks::Dispatcher, worker_init::Hooks};
use crate::cli::A2aArgs;
use anyhow::Result;

pub async fn run(args: A2aArgs) -> Result<()> {
    codetether_native_worker::entrypoints::run::<Hooks, Dispatcher>(args).await
}
