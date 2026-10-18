//! Create the second session for isolation regression tests.
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ServerResponse};

pub(super) async fn create(control: &mut MuxConnection, workspace: &std::path::Path) -> u64 {
    let created = control
        .request(ClientRequest::CreateSession {
            name: "second".into(),
            workspace: workspace.into(),
        })
        .await
        .unwrap();
    let ServerResponse::Snapshot { state } = created else {
        panic!("expected snapshot, got {created:?}");
    };
    let second_window = state.session("second").unwrap().active_window;
    second_window
}
