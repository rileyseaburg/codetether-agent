//! In-process proof that sessions on one server cannot observe each other.

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};
use crate::mux::registry::SessionTarget;

#[tokio::test]
async fn sessions_cannot_see_each_others_windows_or_runtime() {
    let workspace = tempfile::tempdir().unwrap();
    let (first, _, server) = super::pty_support::server_for(workspace.path().into(), 3).await;
    let mut control = MuxConnection::connect_server(&first.record).await.unwrap();
    let created = control
        .request(ClientRequest::CreateSession {
            name: "second".into(),
            workspace: workspace.path().into(),
        })
        .await
        .unwrap();
    let ServerResponse::Snapshot { state } = created else {
        panic!("expected snapshot, got {created:?}");
    };
    let second_window = state.session("second").unwrap().active_window;
    assert_ne!(second_window, first.session().unwrap().active_window);

    let mut one = MuxConnection::connect(&first).await.unwrap();
    let tail = ProgramRequest::Tail {
        window_id: second_window,
    };
    let denied = one
        .request(ClientRequest::Program { request: tail })
        .await
        .unwrap();
    assert!(matches!(denied, ServerResponse::Error { .. }), "{denied:?}");

    let second = SessionTarget {
        record: first.record.clone(),
        session: "second".into(),
    };
    let mut two = MuxConnection::connect(&second).await.unwrap();
    let status = Some(super::fixture::runtime("durable-two"));
    two.request(ClientRequest::ReportRuntime { status })
        .await
        .unwrap();
    let ServerResponse::Snapshot { state } = one.request(ClientRequest::Snapshot).await.unwrap()
    else {
        panic!("expected snapshot");
    };
    assert!(state.session(&first.session).unwrap().runtime.is_none());
    let reported = state.session("second").unwrap().runtime.as_ref().unwrap();
    assert_eq!(reported.session_id, "durable-two");

    for client in [&mut control, &mut one, &mut two] {
        client.request(ClientRequest::Detach).await.unwrap();
    }
    server.await.unwrap();
}
