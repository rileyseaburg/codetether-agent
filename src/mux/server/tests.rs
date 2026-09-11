use tokio::net::TcpListener;

use crate::mux::client::MuxConnection;
use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::{ClientRequest, ServerResponse};

mod coordination;
mod coordination_identity;
mod fixture;
mod idle_benchmark;
mod isolation;
mod isolation_close;
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
    let state = MuxSnapshot::new(
        "network".into(),
        std::env::temp_dir(),
        crate::mux::isolation::Isolation::Worktree,
    );
    let context = super::context::ServerContext::new(state.clone(), "secret".into(), address);
    let task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        super::connection::handle(stream, context).await.unwrap();
    });
    let target = fixture::target(state, address, "network");
    let mut client = MuxConnection::connect(&target).await.unwrap();
    let response = client.request(ClientRequest::Snapshot).await.unwrap();
    assert!(matches!(
        response,
        ServerResponse::Snapshot { state } if state.session("network").is_some()
    ));
    client.request(ClientRequest::Detach).await.unwrap();
    task.await.unwrap();
}
