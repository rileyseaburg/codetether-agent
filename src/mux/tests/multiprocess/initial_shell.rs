//! Multi-process proof that new sessions own a login shell immediately.

use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn new_session_starts_with_persistent_shell() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    tokio::fs::create_dir(&workspace).await.unwrap();
    let mut server = super::process::start("initial-shell", &workspace, root.path()).await;
    let mut client = crate::mux::client::MuxConnection::connect(&server.record)
        .await
        .unwrap();

    let response = client
        .request(ClientRequest::AttachProgram {
            window_id: 0,
            columns: 80,
            rows: 24,
        })
        .await
        .unwrap();
    assert!(matches!(response, ServerResponse::ProgramAttached { .. }));
    super::verify::shutdown(&mut client, &mut server.child).await;
}
