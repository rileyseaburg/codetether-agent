//! Existing crash upload backoff checks.
use super::*;
#[test]
fn backoff_increases_and_clamps() {
    // First failure uses the minimum.
    assert_eq!(compute_backoff_secs(1), MIN_UPLOAD_BACKOFF_SECS);
    // Second failure doubles.
    assert_eq!(compute_backoff_secs(2), MIN_UPLOAD_BACKOFF_SECS * 2);
    // A larger number should never exceed the max cap.
    assert_eq!(compute_backoff_secs(10_000), MAX_UPLOAD_BACKOFF_SECS);
}
