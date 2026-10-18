//! Text-event timing integration regression.
use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn text_events_record_request_ttft_and_last_token() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    app.state.request_timing.processing_started_at =
        Some(std::time::Instant::now() - std::time::Duration::from_millis(15));
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::TextChunk("hello".to_string()),
    )
    .await;
    let first = app
        .state
        .request_timing
        .current_request_first_token_ms
        .unwrap();
    assert_eq!(
        app.state.request_timing.current_request_last_token_ms,
        Some(first)
    );
    app.state.request_timing.processing_started_at =
        Some(std::time::Instant::now() - std::time::Duration::from_millis(30));
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::TextComplete("hello".to_string()),
    )
    .await;
    assert!(
        app.state
            .request_timing
            .current_request_last_token_ms
            .unwrap()
            >= first
    );
}
