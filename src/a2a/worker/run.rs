//! Standalone A2A worker entrypoint.

use anyhow::Result;

use crate::cli::A2aArgs;

use super::{bootstrap_worker, init_worker, run_worker_loop};

pub async fn run(args: A2aArgs) -> Result<()> {
    let context = init_worker(args).await?;
    bootstrap_worker(&context).await?;
    run_worker_loop(context).await
}
