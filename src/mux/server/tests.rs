use chrono::Utc;
use tokio::net::TcpListener;

use crate::mux::client::MuxConnection;
use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::{ClientRequest, ServerResponse};
use crate::mux::registry::MuxRecord;

mod coordination;
mod coordination_identity;
mod idle_benchmark;
mod long_poll;
mod pty;
mod pty_io;
mod pty_replay;
mod pty_support;
mod requests;

#[tokio::test]
async fn authenticated_client_reads_server_snapshot() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let state = MuxSnapshot::new("network".into(), std::env::temp_dir());
    let context = super::context::ServerContext::new(state.clone(), "secret".into(), address);
    let task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        super::connection::handle(stream, context).await.unwrap();
    });
    let record = MuxRecord {
        name: "network".into(),
        address,
        token: "secret".into(),
        pid: 1,
        started_at: Utc::now(),
        state,
    };
    let mut client = MuxConnection::connect(&record).await.unwrap();
    let response = client.request(ClientRequest::Snapshot).await.unwrap();
    assert!(matches!(response, ServerResponse::Snapshot { state } if state.name == "network"));
    client.request(ClientRequest::Detach).await.unwrap();
    task.await.unwrap();
}
