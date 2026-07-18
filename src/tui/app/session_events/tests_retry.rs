use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn stream_retry_clears_failed_preview_and_reports_reconnect() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    app.state.streaming_text = "discarded partial".into();
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::StreamRetry(crate::session::StreamRetryEvent {
            attempt: 2,
            max_restarts: 3,
            reason: "connection reset".into(),
        }),
    )
    .await;
    assert!(app.state.streaming_text.is_empty());
    assert_eq!(app.state.status, "Reconnecting… 2/3");
}
