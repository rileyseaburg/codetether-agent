//! Existing authorization regression case.
use super::*;

#[test]
fn viewer_cannot_access_admin() {
    assert!(!evaluate_local(&test_viewer(), "admin:access"));
}
