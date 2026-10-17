//! Existing test_rate_limit_info_from_headers test.
use super::*;

#[test]
fn test_rate_limit_info_from_headers() {
    let mut headers = HashMap::new();
    headers.insert("x-ratelimit-remaining".to_string(), "45".to_string());
    headers.insert("x-ratelimit-limit".to_string(), "100".to_string());
    headers.insert("x-ratelimit-reset-after".to_string(), "60".to_string());
    headers.insert("retry-after".to_string(), "5".to_string());

    let info = RateLimitInfo::from_headers(&headers);

    assert_eq!(info.remaining, Some(45));
    assert_eq!(info.limit, Some(100));
    assert_eq!(info.reset_after_secs, Some(60));
    assert_eq!(info.retry_after_secs, Some(5));
}
