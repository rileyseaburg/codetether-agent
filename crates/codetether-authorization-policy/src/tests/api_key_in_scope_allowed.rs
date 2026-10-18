//! Existing authorization regression case.
use super::*;

#[test]
fn api_key_in_scope_allowed() {
    assert!(evaluate_local(&test_api_key_user(), "tasks:read"));
}
