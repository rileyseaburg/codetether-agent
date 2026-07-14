//! Listener lifecycle for one named mux server.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Result, bail};
use tokio::net::TcpListener;
use tokio::task::JoinSet;

use crate::mux::model::MuxSnapshot;
use crate::mux::registry;

use super::context::ServerContext;

pub(in crate::mux) async fn serve(
    name: String,
    workspace: PathBuf,
    bind: SocketAddr,
) -> Result<()> {
    if !bind.ip().is_loopback() {
        bail!("mux currently requires a loopback bind; use an SSH tunnel remotely");
    }
    let token = std::env::var(crate::mux::token::BOOTSTRAP_ENV)
        .map_err(|_| anyhow::anyhow!("missing mux bootstrap token"))?;
    let listener = TcpListener::bind(bind).await?;
    let context = ServerContext::new(
        MuxSnapshot::new(name.clone(), workspace),
        token,
        listener.local_addr()?,
    );
    context.persist().await?;
    tracing::info!(session = %name, address = %context.address, "Mux server listening");
    let mut clients = JoinSet::new();
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
            () = context.shutdown.notified() => break,
        }
    }
    clients.abort_all();
    while clients.join_next().await.is_some() {}
    registry::remove(&name).await?;
    tracing::info!(session = %name, "Mux server stopped");
    Ok(())
}
