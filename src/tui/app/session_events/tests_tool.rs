use super::super::*;
use crate::session::{Session, SessionEvent};
use crate::tui::chat::message::MessageType;

#[tokio::test]
async fn tool_completion_records_duration_for_chat_and_latency_view() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::ToolCallComplete {
            name: "read".to_string(),
            output: "src/main.rs".to_string(),
            success: true,
            duration_ms: 42,
        },
    )
    .await;
    let Some(message) = app.state.messages.last() else {
        panic!("expected a tool result message");
    };
    match &message.message_type {
        MessageType::ToolResult {
            name,
            success,
            duration_ms,
            ..
        } => {
            assert_eq!(name, "read");
            assert!(*success);
            assert_eq!(*duration_ms, Some(42));
        }
        other => panic!("expected tool result message, got {other:?}"),
    }
    assert_eq!(app.state.last_tool_name.as_deref(), Some("read"));
    assert_eq!(app.state.last_tool_latency_ms, Some(42));
    assert_eq!(app.state.last_tool_success, Some(true));
}
