//! Complete replay proof for programs that exit while detached.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn exited_program_replays_every_buffered_chunk_after_reconnect() {
    let workspace = tempfile::tempdir().unwrap();
    let (record, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut first = MuxConnection::connect(&record).await.unwrap();
    first
        .request(ClientRequest::StartProgram {
            window_id: 0,
            command: "head -c 70000 /dev/zero | tr '\\000' x".into(),
            columns: 80,
            rows: 24,
        })
        .await
        .unwrap();
    first.request(ClientRequest::Detach).await.unwrap();
    super::pty_io::wait_for_exit(&context).await;

    let mut second = MuxConnection::connect(&record).await.unwrap();
    let attached = second
        .request(ClientRequest::AttachProgram {
            window_id: 0,
            columns: 80,
            rows: 24,
        })
        .await
        .unwrap();
    let ServerResponse::ProgramAttached { mut offset, .. } = attached else {
        panic!()
    };
    let output = super::pty_io::read_all(&mut second, &mut offset).await;
    assert_eq!(output.len(), 70_000);
    second.request(ClientRequest::Detach).await.unwrap();
    server.await.unwrap();
}
