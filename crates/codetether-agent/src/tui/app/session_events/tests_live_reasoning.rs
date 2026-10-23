use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn live_reasoning_replaces_preview_then_commits_once() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    for text in ["Checking", "Checking the dependencies"] {
        handle_session_event(
            &mut app,
            &mut slot,
            &None,
            SessionEvent::ThinkingChunk(text.into()),
        )
        .await;
        assert_eq!(app.state.streaming.streaming_reasoning, text);
        assert!(app.state.messages.is_empty());
        assert!(app.state.processing);
    }
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::ThinkingComplete("Checking the dependencies".into()),
    )
    .await;
    assert!(app.state.streaming.streaming_reasoning.is_empty());
    assert_eq!(app.state.messages.len(), 1);
    assert_eq!(app.state.messages[0].content, "Checking the dependencies");
}

#[path = "tests_live_reasoning_clear.rs"]
mod clear;
