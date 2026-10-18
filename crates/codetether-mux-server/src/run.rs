//! Listener lifecycle for one named mux server.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Result, bail};
use tokio::net::TcpListener;

use codetether_mux_registry as registry;

use super::client_tasks::ClientTasks;

/// Serve an authenticated mux workspace until a shutdown request.
///
/// # Arguments
/// * `name` - Initial session name.
/// * `workspace` - Initial workspace directory.
/// * `bind` - Loopback address for the listener.
/// * `isolation` - Workspace allocation policy for new windows.
/// # Returns
/// Returns after closing client tasks and server-owned processes.
/// # Errors
/// Returns binding, initialization, persistence or connection-accept errors.
/// The bootstrap token must be supplied through [crate::token::BOOTSTRAP_ENV].
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_server::serve("work".into(), ".".into(), "127.0.0.1:0".parse()?,
///     codetether_mux_workspace::Isolation::Shared).await
/// # }
/// ```
pub async fn serve(
    name: String,
    workspace: PathBuf,
    bind: SocketAddr,
    isolation: codetether_mux_workspace::Isolation,
) -> Result<()> {
    if !bind.ip().is_loopback() {
        bail!("mux currently requires a loopback bind; use an SSH tunnel remotely");
    }
    let listener = TcpListener::bind(bind).await?;
    let context =
        super::startup::initialize(&name, workspace, listener.local_addr()?, isolation).await?;
    let key = context.key().await;
    tracing::info!(session = %name, workspace = %key, address = %context.address, "Mux server listening");
    let mut clients = ClientTasks::new();
    loop {
        tokio::select! {
            accepted = listener.accept() => {
                let (stream, peer) = accepted?;
                let context = context.clone();
                clients.spawn(async move {
                    if let Err(error) = super::connection::handle(stream, context).await {
                        tracing::debug!(%peer, %error, "Mux client disconnected");
                    }
                });
            }
            () = clients.reap() => {}
            () = context.shutdown.notified() => break,
        }
    }
    clients.shutdown().await;
    context.tasks.cancel_all();
    context.programs.stop_all();
    registry::remove_key(&key).await?;
    tracing::info!(workspace = %key, "Mux server stopped");
    Ok(())
}
