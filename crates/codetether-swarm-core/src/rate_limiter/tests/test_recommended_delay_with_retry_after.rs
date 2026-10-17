//! Existing test_recommended_delay_with_retry_after test.
use super::*;

#[test]
fn test_recommended_delay_with_retry_after() {
    let info = RateLimitInfo {
        retry_after_secs: Some(10),
        ..Default::default()
    };
    assert_eq!(info.recommended_delay(), Duration::from_secs(10));
}
