//! Existing authorization regression case.
use super::*;

#[test]
fn a2a_admin_inherits_admin() {
    let user = test_user(&["a2a-admin"], &[], "keycloak");
    assert!(evaluate_local(&user, "admin:access"));
}
