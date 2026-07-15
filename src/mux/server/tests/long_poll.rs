//! Concurrent input and event-driven output regression coverage.

use std::time::Duration;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn pending_output_read_wakes_after_control_connection_input() {
    let workspace = tempfile::tempdir().unwrap();
    let (record, context, server) = super::pty_support::server(workspace.path().into()).await;
    let mut control = MuxConnection::connect(&record).await.unwrap();
    control
        .request(ClientRequest::StartProgram {
            window_id: 0,
            command: "stty -echo; read value; printf got:$value; sleep 1".into(),
            columns: 80,
            rows: 24,
        })
        .await
        .unwrap();
    let mut output = MuxConnection::connect(&record).await.unwrap();
    let reader = tokio::spawn(async move {
        let response = output
            .request(ClientRequest::ReadProgram {
                window_id: 0,
                offset: 0,
            })
            .await;
        output.request(ClientRequest::Detach).await.unwrap();
        response.unwrap()
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    control
        .request(ClientRequest::ProgramInput {
            window_id: 0,
            data: b"proof\n".to_vec(),
        })
        .await
        .unwrap();
    let response = tokio::time::timeout(Duration::from_millis(500), reader)
        .await
        .unwrap()
        .unwrap();
    let ServerResponse::ProgramOutput { data, .. } = response else {
        panic!()
    };
    assert!(String::from_utf8_lossy(&data).contains("got:proof"));
    control.request(ClientRequest::Detach).await.unwrap();
    context.programs.stop_all();
    server.await.unwrap();
}
