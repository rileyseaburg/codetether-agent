//! Shared two-client mux server fixture for PTY tests.

use std::path::PathBuf;
use std::sync::Arc;

use chrono::Utc;
use tokio::net::TcpListener;

use crate::mux::model::MuxSnapshot;
use crate::mux::registry::MuxRecord;

use super::super::context::ServerContext;

pub(super) async fn server(
    workspace: PathBuf,
) -> (MuxRecord, Arc<ServerContext>, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let state = MuxSnapshot::new("pty-proof".into(), workspace);
    let context = ServerContext::new(state.clone(), "secret".into(), address);
    let server_context = context.clone();
    let task = tokio::spawn(async move {
        let mut clients = tokio::task::JoinSet::new();
        for _ in 0..2 {
            let (stream, _) = listener.accept().await.unwrap();
            let context = server_context.clone();
            clients.spawn(async move { super::super::connection::handle(stream, context).await });
        }
        while let Some(result) = clients.join_next().await {
            result.unwrap().unwrap();
        }
    });
    let record = MuxRecord {
        name: "pty-proof".into(),
        address,
        token: "secret".into(),
        pid: 1,
        started_at: Utc::now(),
        state,
    };
    (record, context, task)
}
