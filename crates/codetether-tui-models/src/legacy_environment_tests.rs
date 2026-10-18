//! Restored explicit-flag and deployment-name regression cases.
use super::legacy_environment::is_secure_environment_from_values;

#[test]
fn secure_environment_detection_respects_explicit_flags() {
    assert!(is_secure_environment_from_values(Some(true), None, None));
    assert!(!is_secure_environment_from_values(
        Some(false),
        Some(true),
        Some("secure")
    ));
}

#[test]
fn secure_environment_detection_uses_environment_name_fallback() {
    assert!(is_secure_environment_from_values(None, None, Some("PROD")));
    assert!(is_secure_environment_from_values(
        None,
        None,
        Some("production")
    ));
    assert!(!is_secure_environment_from_values(None, None, Some("dev")));
}
