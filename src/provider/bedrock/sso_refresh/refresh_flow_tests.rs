//! Tests for refresh-token rotation detection in the silent SSO flow.

use super::rotated_token;

#[test]
fn absent_token_means_no_rotation() {
    assert_eq!(rotated_token(None, "old"), None);
}

#[test]
fn identical_token_means_no_rotation() {
    assert_eq!(rotated_token(Some("old".into()), "old"), None);
}

#[test]
fn empty_token_is_ignored() {
    assert_eq!(rotated_token(Some(String::new()), "old"), None);
}

#[test]
fn different_token_is_reported_for_persistence() {
    assert_eq!(
        rotated_token(Some("new".into()), "old"),
        Some("new".to_string())
    );
}
