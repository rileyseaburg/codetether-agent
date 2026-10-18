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
            tool_call_id: "call-1".into(),
            name: "read".into(),
            arguments: "{\"path\":\"src/lib.rs\"}".into(),
        },
    )
    .await;
    assert!(app.state.status.contains("Running read"));
    assert!(app.state.status.contains("src/lib.rs"));
}

#[tokio::test]
async fn approval_required_tool_result_surfaces_approve_command() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    let output = r#"{"error":{"approval_request_id":"approval-1"}}"#;
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::ToolCallComplete {
            tool_call_id: "call-1".into(),
            name: "bash".into(),
            output: output.into(),
            success: false,
            duration_ms: 7,
        },
    )
    .await;
    assert!(app.state.status.contains("/approve approval-1"));
    assert!(app.state.status.contains("Access mode"));
    assert!(!app.state.status.contains("retry"));
}
