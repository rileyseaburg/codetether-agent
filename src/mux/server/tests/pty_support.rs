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
        for _ in 0..2 {
            let (stream, _) = listener.accept().await.unwrap();
            super::super::connection::handle(stream, server_context.clone())
                .await
                .unwrap();
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
