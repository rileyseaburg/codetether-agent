//! Two-client PTY server fixture.
use super::super::context::ServerContext;
use codetether_mux_registry::SessionTarget;
use std::path::PathBuf;
use std::sync::Arc;

pub(super) async fn server(
    workspace: PathBuf,
) -> (
    SessionTarget,
    Arc<ServerContext>,
    tokio::task::JoinHandle<()>,
) {
    super::pty_support::server_for(workspace, 2).await
}
