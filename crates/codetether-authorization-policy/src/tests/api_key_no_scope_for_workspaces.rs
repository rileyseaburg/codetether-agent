//! Existing authorization regression case.
use super::*;

#[test]
fn api_key_no_scope_for_workspaces() {
    assert!(!evaluate_local(&test_api_key_user(), "workspaces:read"));
}
