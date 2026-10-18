//! Tests for Bedrock 401/403 recovery decisions (pure parts only).

use super::{adoptable, is_auth_failure};
use reqwest::StatusCode;

#[test]
fn only_401_and_403_count_as_auth_failures() {
    assert!(is_auth_failure(StatusCode::UNAUTHORIZED));
    assert!(is_auth_failure(StatusCode::FORBIDDEN));
    assert!(!is_auth_failure(StatusCode::TOO_MANY_REQUESTS));
    assert!(!is_auth_failure(StatusCode::INTERNAL_SERVER_ERROR));
    assert!(!is_auth_failure(StatusCode::OK));
}

#[test]
fn adopts_a_different_stored_key() {
    assert_eq!(adoptable(Some("fresh"), "stale"), Some("fresh".to_string()));
}

#[test]
fn does_not_adopt_the_same_rejected_key() {
    assert_eq!(adoptable(Some("stale"), "stale"), None);
}

#[test]
fn ignores_missing_or_empty_stored_key() {
    assert_eq!(adoptable(None, "stale"), None);
    assert_eq!(adoptable(Some(""), "stale"), None);
}
