//! Existing streaming HTTP error regressions.

use super::*;

#[test]
fn forbidden_appends_refresh_guidance() {
    let msg = message(StatusCode::FORBIDDEN, "Authentication failed");
    assert!(msg.contains("403"));
    assert!(msg.contains("codetether auth bedrock --refresh"));
}

#[test]
fn unauthorized_appends_refresh_guidance() {
    assert!(message(StatusCode::UNAUTHORIZED, "bad key").contains("--refresh"));
}

#[test]
fn other_status_has_no_refresh_guidance() {
    let msg = message(StatusCode::INTERNAL_SERVER_ERROR, "boom");
    assert!(msg.contains("500"));
    assert!(!msg.contains("--refresh"));
}
