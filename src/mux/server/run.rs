//! Listener lifecycle for one named mux server.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Result, bail};
use tokio::net::TcpListener;

use crate::mux::registry;

use super::client_tasks::ClientTasks;

pub(in crate::mux) async fn serve(
    name: String,
    workspace: PathBuf,
    bind: SocketAddr,
    isolation: crate::mux::isolation::Isolation,
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
