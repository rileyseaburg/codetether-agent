//! Initial server context and persistent login-shell startup.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::mux::model::MuxSnapshot;
use crate::mux::pty::TerminalSize;

use super::context::ServerContext;

pub(super) async fn initialize(
    name: &str,
    workspace: PathBuf,
    address: SocketAddr,
    isolation: crate::mux::isolation::Isolation,
) -> Result<Arc<ServerContext>> {
    let token = std::env::var(crate::mux::token::BOOTSTRAP_ENV)
        .map_err(|_| anyhow::anyhow!("missing mux bootstrap token"))?;
    let workspace = tokio::fs::canonicalize(&workspace)
        .await
        .unwrap_or(workspace);
    let state = MuxSnapshot::new(name.into(), workspace.clone(), isolation);
    let context = ServerContext::new(state, token, address);
    context.programs.start(
        0,
        &crate::mux::pty::default_shell::command(),
        &workspace,
        TerminalSize::new(80, 24),
        name,
    )?;
    context.persist().await?;
    Ok(context)
}
