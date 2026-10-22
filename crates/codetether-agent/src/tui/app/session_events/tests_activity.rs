use super::super::*;
use crate::session::SessionEvent;
use std::time::SystemTime;

#[tokio::test]
async fn activity_age_resets_for_thinking_messages_and_tools() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    let events = [
        SessionEvent::Thinking,
        SessionEvent::TextChunk("hello".into()),
        SessionEvent::ThinkingComplete("reasoning".into()),
        SessionEvent::ToolCallStart {
            tool_call_id: "activity-test".into(),
            name: "read".into(),
            arguments: "{}".into(),
        },
        SessionEvent::ToolHeartbeat {
            tool_call_id: "activity-test".into(),
            name: "read".into(),
            elapsed_secs: 1,
        },
        SessionEvent::ToolCallComplete {
            tool_call_id: "activity-test".into(),
            name: "read".into(),
            output: "ok".into(),
            success: true,
            duration_ms: 1000,
        },
    ];
    for event in events {
        app.state.last_activity_at = SystemTime::UNIX_EPOCH;
        let before = SystemTime::now();
        handle_session_event(&mut app, &mut slot, &None, event).await;
        assert!(app.state.last_activity_at >= before);
    }
}
