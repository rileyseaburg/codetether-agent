//! Existing authorization regression case.
use super::*;

#[test]
fn admin_can_access_admin() {
    assert!(evaluate_local(&test_admin(), "admin:access"));
}
