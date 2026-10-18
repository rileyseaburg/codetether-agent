//! Existing authorization regression case.
use super::*;

#[test]
fn public_endpoint_always_allowed() {
    let no_roles = PolicyUser {
        user_id: "anon".to_string(),
        roles: vec![],
        tenant_id: None,
        scopes: vec![],
        auth_source: "keycloak".to_string(),
    };
    assert!(evaluate_local(&no_roles, "health"));
}
