//! Existing test_is_approaching_limit test.
use super::*;

#[test]
fn test_is_approaching_limit() {
    let info = RateLimitInfo {
        remaining: Some(15),
        limit: Some(100),
        ..Default::default()
    };
    assert!(info.is_approaching_limit());

    let info2 = RateLimitInfo {
        remaining: Some(50),
        limit: Some(100),
        ..Default::default()
    };
    assert!(!info2.is_approaching_limit());
}
