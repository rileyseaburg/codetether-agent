//! Unit tests for retry classification helpers.

use super::classify::{is_permanent_message, is_retryable_message, is_retryable_status};
use reqwest::StatusCode;

#[test]
fn expired_glm_plan_is_permanent_not_retryable() {
    // Real Z.AI 429 body: code 1309, expired coding plan.
    let body = r#"{"error":{"code":"1309","message":"Your GLM Coding Plan package has expired and is temporarily unavailable. You can resume using it after renewing the subscription."}}"#;
    assert!(is_permanent_message(body));
    // Status alone still looks retryable; the body is what disqualifies it.
    assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
}

#[test]
fn genuine_throttling_is_not_permanent() {
    assert!(!is_permanent_message(
        "Too Many Requests: rate limit exceeded"
    ));
    assert!(is_retryable_message("rate limit exceeded"));
}

#[test]
fn insufficient_balance_is_permanent() {
    assert!(is_permanent_message("Insufficient balance in your account"));
}
