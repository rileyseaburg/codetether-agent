//! Existing authorization policy regression cases.
use crate::{PolicyUser, config::sidecar_disabled, local::evaluate_local};

fn test_user(roles: &[&str], scopes: &[&str], auth_source: &str) -> PolicyUser {
    PolicyUser {
        user_id: "test-user".to_string(),
        roles: roles.iter().copied().map(str::to_owned).collect(),
        tenant_id: Some("t1".to_string()),
        scopes: scopes.iter().copied().map(str::to_owned).collect(),
        auth_source: auth_source.to_string(),
    }
}

fn test_admin() -> PolicyUser {
    test_user(&["admin"], &[], "keycloak")
}

fn test_viewer() -> PolicyUser {
    test_user(&["viewer"], &[], "keycloak")
}

fn test_api_key_user() -> PolicyUser {
    test_user(&["editor"], &["tasks:read", "tasks:write"], "api_key")
}
mod a2a_admin_inherits_admin;
mod admin_can_access_admin;
mod api_key_in_scope_allowed;
mod api_key_no_scope_for_workspaces;
mod api_key_out_of_scope_denied;
mod opa_enabled_false_disables_sidecar;
mod public_endpoint_always_allowed;
mod viewer_can_read_tasks;
mod viewer_cannot_access_admin;
mod viewer_cannot_write_tasks;
