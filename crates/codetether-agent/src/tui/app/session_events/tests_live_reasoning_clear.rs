use super::*;

#[tokio::test]
async fn live_reasoning_clears_when_reply_starts() {
    let mut app = App::default();
    let mut slot = super::super::test_slot().await;
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::ThinkingChunk("working".into()),
    )
    .await;
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::TextChunk("answer".into()),
    )
    .await;
    assert!(app.state.streaming.streaming_reasoning.is_empty());
    assert_eq!(app.state.streaming.streaming_text, "answer");
}
