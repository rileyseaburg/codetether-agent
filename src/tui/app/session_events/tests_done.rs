use super::super::*;
use crate::session::SessionEvent;

#[tokio::test]
async fn done_promotes_request_timing_snapshot() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    app.state.processing_started_at = Some(std::time::Instant::now());
    app.state.current_request_first_token_ms = Some(120);
    app.state.current_request_last_token_ms = Some(980);
    handle_session_event(&mut app, &mut slot, &None, SessionEvent::Done).await;
    assert_eq!(app.state.last_request_first_token_ms, Some(120));
    assert_eq!(app.state.last_request_last_token_ms, Some(980));
    assert!(app.state.processing_started_at.is_none());
    assert!(app.state.current_request_first_token_ms.is_none());
    assert!(app.state.current_request_last_token_ms.is_none());
}

#[tokio::test]
async fn done_clears_watchdog_notification_to_stop_resubmit_loop() {
    use crate::tui::app::watchdog::WatchdogNotification;
    let mut app = App::default();
    let mut slot = super::test_slot().await;

    // Simulate a watchdog cancel that left a pending notification + inflight
    // prompt. Before the fix, Done reset the restart count but left the
    // notification set, so the retry guard resubmitted the same prompt forever.
    app.state.watchdog_notification =
        Some(WatchdogNotification::new("stalled".to_string(), 1));
    app.state.main_inflight_prompt = Some("hello".to_string());
    app.state.main_watchdog_restart_count = 1;

    handle_session_event(&mut app, &mut slot, &None, SessionEvent::Done).await;

    assert!(app.state.watchdog_notification.is_none());
    assert!(app.state.main_inflight_prompt.is_none());
    // Count intentionally survives a completed turn so a completes-then-stalls
    // storm still climbs to the give-up cap; it is reset on new user prompts.
    assert_eq!(app.state.main_watchdog_restart_count, 1);
}
