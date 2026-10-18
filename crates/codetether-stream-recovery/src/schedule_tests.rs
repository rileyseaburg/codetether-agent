//! Tests for stream-reconnect scheduling.

use crate::StreamRecoveryState;

#[test]
fn schedule_increments_count_and_stores_prompt() {
    let mut state = StreamRecoveryState::default();
    let prompt = Some("hello".into());
    state.schedule("connection reset by peer", &prompt);
    assert_eq!(state.stream_reconnect_count, 1);
    assert_eq!(state.pending_stream_reconnect.as_deref(), Some("hello"));
}

#[test]
fn schedule_ignores_permanent_errors() {
    let mut state = StreamRecoveryState::default();
    let prompt = Some("hello".into());
    state.schedule("invalid api key", &prompt);
    assert_eq!(state.stream_reconnect_count, 0);
    assert!(state.pending_stream_reconnect.is_none());
}

#[test]
fn reset_on_success_clears_state() {
    let mut state = StreamRecoveryState::default();
    state.stream_reconnect_count = 2;
    state.pending_stream_reconnect = Some("prompt".into());
    state.reset_on_success();
    assert_eq!(state.stream_reconnect_count, 0);
    assert!(state.pending_stream_reconnect.is_none());
}
