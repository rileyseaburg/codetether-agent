//! Existing test_validate_url_always_uses_api_base test.
use super::*;

#[test]
fn test_validate_url_always_uses_api_base() {
    let url = MoltbookClient::validate_url("/agents/me");
    assert!(url.starts_with("https://www.moltbook.com/api/v1"));
    assert!(!url.contains("http://"));
}
