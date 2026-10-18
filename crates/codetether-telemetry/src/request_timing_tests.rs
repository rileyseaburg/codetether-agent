//! Regression coverage for first/last-token timing and completion.
use super::RequestTimingState;
use std::time::{Duration, Instant};

#[test]
fn request_timing_captures_first_and_last_token() {
    let mut state = RequestTimingState::default();
    state.processing_started_at = Some(
        Instant::now()
            .checked_sub(Duration::from_millis(12))
            .unwrap(),
    );
    state.note_text_token();
    let first = state
        .current_request_first_token_ms
        .expect("first token should be recorded");
    assert_eq!(state.current_request_last_token_ms, Some(first));
    state.processing_started_at = Some(
        Instant::now()
            .checked_sub(Duration::from_millis(34))
            .unwrap(),
    );
    state.note_text_token();
    let last = state
        .current_request_last_token_ms
        .expect("last token should be recorded");
    assert!(last >= first);
    state.complete_request_timing();
    assert_eq!(state.last_request_first_token_ms, Some(first));
    assert_eq!(state.last_request_last_token_ms, Some(last));
    assert!(state.processing_started_at.is_none());
    assert!(state.current_request_first_token_ms.is_none());
    assert!(state.current_request_last_token_ms.is_none());
}
