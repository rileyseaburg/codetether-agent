use super::super::*;
use crate::session::{Session, SessionEvent};

#[tokio::test]
async fn text_chunk_replaces_streaming_preview_with_latest_cumulative_text() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::TextChunk("hel".to_string()),
    )
    .await;
    assert_eq!(app.state.streaming_text, "hel");
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::TextChunk("hello".to_string()),
    )
    .await;
    assert_eq!(app.state.streaming_text, "hello");
}

#[tokio::test]
async fn text_events_record_request_ttft_and_last_token() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");
    app.state.processing_started_at =
        Some(std::time::Instant::now() - std::time::Duration::from_millis(15));
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::TextChunk("hello".to_string()),
    )
    .await;
    let first = app.state.current_request_first_token_ms.unwrap();
    assert_eq!(app.state.current_request_last_token_ms, Some(first));
    app.state.processing_started_at =
        Some(std::time::Instant::now() - std::time::Duration::from_millis(30));
    handle_session_event(
        &mut app,
        &mut session,
        &None,
        SessionEvent::TextComplete("hello".to_string()),
    )
    .await;
    assert!(app.state.current_request_last_token_ms.unwrap() >= first);
}
