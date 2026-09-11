//! Closing sessions keeps the server alive until the last one goes.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};

#[tokio::test]
async fn last_session_close_stops_the_server() {
    let workspace = tempfile::tempdir().unwrap();
    let (first, context, server) = super::pty_support::server_for(workspace.path().into(), 1).await;
    let mut control = MuxConnection::connect_server(&first.record).await.unwrap();
    let created = control
        .request(ClientRequest::CreateSession {
            name: "second".into(),
            workspace: workspace.path().into(),
        })
        .await
        .unwrap();
    assert!(matches!(created, ServerResponse::Snapshot { .. }));
    assert_eq!(context.state.read().await.sessions.len(), 2);

    let kept = control
        .request(ClientRequest::CloseSession {
            name: "second".into(),
        })
        .await
        .unwrap();
    assert!(matches!(kept, ServerResponse::Snapshot { .. }));
    assert_eq!(context.state.read().await.sessions.len(), 1);

    let stopped = control
        .request(ClientRequest::CloseSession {
            name: first.session.clone(),
        })
        .await
        .unwrap();
    assert!(matches!(stopped, ServerResponse::ShuttingDown));
    assert!(context.state.read().await.sessions.is_empty());
    server.await.unwrap();
}
