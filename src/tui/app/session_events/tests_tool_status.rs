use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn tool_events_surface_preview_in_status() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::ToolCallStart {
            name: "read".into(),
            arguments: "{\"path\":\"src/lib.rs\"}".into(),
        },
    )
    .await;
    assert!(app.state.status.contains("Running read"));
    assert!(app.state.status.contains("src/lib.rs"));
}
