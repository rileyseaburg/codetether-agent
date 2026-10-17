//! Structured event regression case.
use super::*;

#[test]
fn sensitive_key_detection_is_case_insensitive() {
    assert!(is_sensitive_key("GITHUB_TOKEN"));
    assert!(is_sensitive_key("Authorization"));
    assert!(is_sensitive_key("db_password"));
    assert!(!is_sensitive_key("path"));
    assert!(!is_sensitive_key("command"));
}
