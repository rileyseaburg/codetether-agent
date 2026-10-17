//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_returns_none_for_no_json() {
    assert!(extract_json("no json here").is_none());
}
