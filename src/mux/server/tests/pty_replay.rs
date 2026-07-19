//! Bounded replay proof for programs that produced detached output.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

#[tokio::test]
async fn reconnect_replays_only_the_latest_output_chunk() {
    let workspace = tempfile::tempdir().unwrap();
    let (record, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut first = MuxConnection::connect(&record).await.unwrap();
    first
        .request(ClientRequest::Program {
            request: ProgramRequest::Start {
                window_id: 0,
                command: "head -c 70000 /dev/zero | tr '\\000' x".into(),
                columns: 80,
                rows: 24,
            },
        })
        .await
        .unwrap();
    first.request(ClientRequest::Detach).await.unwrap();
    super::pty_io::wait_for_exit(&context).await;

    let mut second = MuxConnection::connect(&record).await.unwrap();
    let attached = second
        .request(ClientRequest::Program {
            request: ProgramRequest::Attach {
                window_id: 0,
                columns: 80,
                rows: 24,
            },
        })
        .await
        .unwrap();
    let ServerResponse::ProgramAttached { mut offset, .. } = attached else {
        panic!()
    };
    let output = super::pty_io::read_all(&mut second, &mut offset).await;
    assert_eq!(output.len(), 64 * 1024);
    second.request(ClientRequest::Detach).await.unwrap();
    server.await.unwrap();
}
