use super::super::*;
use crate::session::SessionEvent;
use std::time::{Instant, SystemTime};

#[tokio::test]
async fn activity_age_ignores_provider_keepalives() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    handle_session_event(&mut app, &mut slot, &None, SessionEvent::Thinking).await;
    assert_eq!(app.state.last_activity_kind, "thinking");
    app.state.last_activity_at = SystemTime::UNIX_EPOCH;
    app.state.status = "Waiting for model output".into();
    app.state.needs_redraw = false;
    let before = Instant::now();
    handle_session_event(&mut app, &mut slot, &None, SessionEvent::ProviderKeepalive).await;
    assert_eq!(app.state.last_activity_at, SystemTime::UNIX_EPOCH);
    assert_eq!(app.state.last_activity_kind, "thinking");
    assert!(app.state.watchdog.last_event_at.unwrap() >= before);
    assert_eq!(app.state.status, "Waiting for model output");
    assert!(app.state.messages.is_empty());
    assert!(app.state.needs_redraw);
    assert!(
        app.state
            .activity_events
            .entries
            .back()
            .unwrap()
            .label
            .contains("no new output")
    );
}
