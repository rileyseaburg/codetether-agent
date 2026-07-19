//! End-to-end persistence proof across separate network attachments.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn program_survives_detach_and_accepts_input_after_reconnect() {
    let workspace = tempfile::tempdir().unwrap();
    let proof = workspace.path().join("mux-proof.txt");
    let (record, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut first = MuxConnection::connect(&record).await.unwrap();
    let command = "read value; printf '%s' \"$value\" > mux-proof.txt; sleep 5";
    let started = first
        .request(super::requests::start(command))
        .await
        .unwrap();
    assert!(matches!(
        started,
        ServerResponse::ProgramAttached { window_id: 0, .. }
    ));
    first.request(ClientRequest::Detach).await.unwrap();

    let mut second = MuxConnection::connect(&record).await.unwrap();
    let attached = second
        .request(super::requests::attach(100, 30))
        .await
        .unwrap();
    assert!(matches!(
        attached,
        ServerResponse::ProgramAttached { window_id: 0, .. }
    ));
    second
        .request(super::requests::input(b"persisted\n"))
        .await
        .unwrap();
    super::pty_io::wait_for(&proof).await;
    assert_eq!(std::fs::read_to_string(&proof).unwrap(), "persisted");
    second.request(ClientRequest::Detach).await.unwrap();
    context.programs.stop_all();
    server.await.unwrap();
}
