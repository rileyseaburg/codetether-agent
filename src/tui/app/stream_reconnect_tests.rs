//! Tests for stream-reconnect scheduling.

use super::stream_reconnect::{schedule, reset_on_success};
use crate::tui::app::state::App;

#[test]
fn schedule_increments_count_and_stores_prompt() {
    let mut app = App::default();
    app.state.main_inflight_prompt = Some("hello".into());
    schedule(&mut app, "connection reset by peer");
    assert_eq!(app.state.stream_reconnect_count, 1);
    assert_eq!(app.state.pending_stream_reconnect.as_deref(), Some("hello"));
}

#[test]
fn schedule_ignores_permanent_errors() {
    let mut app = App::default();
    app.state.main_inflight_prompt = Some("hello".into());
    schedule(&mut app, "invalid api key");
    assert_eq!(app.state.stream_reconnect_count, 0);
    assert!(app.state.pending_stream_reconnect.is_none());
}

#[test]
fn reset_on_success_clears_state() {
    let mut app = App::default();
    app.state.stream_reconnect_count = 2;
    app.state.pending_stream_reconnect = Some("prompt".into());
    reset_on_success(&mut app);
    assert_eq!(app.state.stream_reconnect_count, 0);
    assert!(app.state.pending_stream_reconnect.is_none());
}
