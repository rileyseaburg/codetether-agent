//! Existing authorization regression case.
use super::*;

#[test]
fn api_key_out_of_scope_denied() {
    assert!(!evaluate_local(&test_api_key_user(), "admin:access"));
}
