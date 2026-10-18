//! Hidden mux server process entry point.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Result;

pub(super) async fn run(
    session: String,
    directory: PathBuf,
    bind: SocketAddr,
    no_worktree: bool,
) -> Result<()> {
    codetether_mux_registry::validate_name(&session)?;
    let directory = tokio::fs::canonicalize(directory).await?;
    let isolation = codetether_mux_workspace::Isolation::from_no_worktree(no_worktree);
    codetether_mux_server::serve(session, directory, bind, isolation).await
}
