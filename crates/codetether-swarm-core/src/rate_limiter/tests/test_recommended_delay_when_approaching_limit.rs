//! Existing test_recommended_delay_when_approaching_limit test.
use super::*;

#[test]
fn test_recommended_delay_when_approaching_limit() {
    let info = RateLimitInfo {
        remaining: Some(5),
        limit: Some(100),
        reset_after_secs: Some(60),
        ..Default::default()
    };
    // Should spread 5 requests over 60 seconds = 12 seconds each
    assert_eq!(info.recommended_delay(), Duration::from_secs(12));
}
