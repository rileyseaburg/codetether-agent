use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn text_chunk_replaces_streaming_preview_with_latest_cumulative_text() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::TextChunk("hel".to_string()),
    )
    .await;
    assert_eq!(app.state.streaming.streaming_text, "hel");
    assert!(app.state.needs_redraw);
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::TextChunk("hello".to_string()),
    )
    .await;
    assert_eq!(app.state.streaming.streaming_text, "hello");
}
