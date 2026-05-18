use super::super::*;
use crate::session::{Session, SessionEvent};

#[tokio::test]
async fn done_promotes_request_timing_snapshot() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");
    app.state.processing_started_at = Some(std::time::Instant::now());
    app.state.current_request_first_token_ms = Some(120);
    app.state.current_request_last_token_ms = Some(980);
    handle_session_event(&mut app, &mut session, &None, SessionEvent::Done).await;
    assert_eq!(app.state.last_request_first_token_ms, Some(120));
    assert_eq!(app.state.last_request_last_token_ms, Some(980));
    assert!(app.state.processing_started_at.is_none());
    assert!(app.state.current_request_first_token_ms.is_none());
    assert!(app.state.current_request_last_token_ms.is_none());
}
