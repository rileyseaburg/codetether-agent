//! Shared mux server fixture for network tests.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpListener;

use codetether_mux_model::MuxSnapshot;
use codetether_mux_registry::SessionTarget;

use super::super::context::ServerContext;

pub(super) const SESSION: &str = "pty-proof";

/// One server hosting `SESSION`, accepting `clients` connections.
pub(super) async fn server_for(
    workspace: PathBuf,
    clients: usize,
) -> (
    SessionTarget,
    Arc<ServerContext>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let state = MuxSnapshot::new(
        SESSION.into(),
        workspace,
        codetether_mux_workspace::Isolation::Worktree,
    );
    let context = ServerContext::new(state.clone(), "secret".into(), address);
    let server_context = context.clone();
    let task = tokio::spawn(async move {
        let mut accepted = tokio::task::JoinSet::new();
        for _ in 0..clients {
            let (stream, _) = listener.accept().await.unwrap();
            let context = server_context.clone();
            accepted.spawn(async move { super::super::connection::handle(stream, context).await });
        }
        while let Some(result) = accepted.join_next().await {
            result.unwrap().unwrap();
        }
    });
    let target = super::fixture::target(state, address, SESSION);
    (target, context, task)
}

pub(super) use super::pty_default_server::server;
