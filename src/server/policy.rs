//! OPA Policy Engine Client
//!
//! Calls the OPA sidecar over HTTP to evaluate authorization decisions.
//! Set `OPA_LOCAL_MODE=true` or `OPA_ENABLED=false` to run in local mode
//! using the compiled-in role → permission mappings from `policies/data.json`.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::time::Duration;
use tracing;

/// OPA sidecar URL.  Defaults to the K8s sidecar address.
fn opa_url() -> String {
    std::env::var("OPA_URL").unwrap_or_else(|_| "http://localhost:8181".to_string())
}

/// OPA query path for the combined authz + api-key scope policy.
fn opa_path() -> String {
    std::env::var("OPA_AUTHZ_PATH").unwrap_or_else(|_| "v1/data/api_keys/allow".to_string())
}

/// Whether to fail open (allow) when OPA is unreachable.
fn fail_open() -> bool {
    std::env::var("OPA_FAIL_OPEN").is_ok_and(|value| value.eq_ignore_ascii_case("true"))
}

fn sidecar_disabled(local_mode: bool, opa_enabled: Option<&str>) -> bool {
    local_mode || opa_enabled.is_some_and(|value| value.eq_ignore_ascii_case("false"))
}

/// Whether to evaluate policies locally without an OPA sidecar.
fn local_mode() -> bool {
    let opa_enabled = std::env::var("OPA_ENABLED").ok();
    sidecar_disabled(
        std::env::var("OPA_LOCAL_MODE").is_ok_and(|value| value.eq_ignore_ascii_case("true")),
        opa_enabled.as_deref(),
    )
}

// ─── Shared HTTP client ──────────────────────────────────────────

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(2))
        .pool_max_idle_per_host(4)
        .build()
        .expect("failed to build reqwest client")
});

// ─── Input / Output types ────────────────────────────────────────

/// User context passed into the OPA input document.
#[derive(Debug, Clone, Serialize)]
pub struct PolicyUser {
    pub user_id: String,
    pub roles: Vec<String>,
    pub tenant_id: Option<String>,
    pub scopes: Vec<String>,
    pub auth_source: String,
}

/// Resource context (optional).
#[derive(Debug, Clone, Default, Serialize)]
pub struct PolicyResource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Serialize)]
struct OpaInput {
    input: OpaInputBody,
}

#[derive(Serialize)]
struct OpaInputBody {
    user: PolicyUser,
    action: String,
    resource: PolicyResource,
}

#[derive(Deserialize)]
struct OpaResponse {
    result: Option<bool>,
}

// ─── Local policy data (compiled in) ─────────────────────────────

/// Embedded copy of `policies/data.json` for local evaluation.
static POLICY_DATA: &str = include_str!("../../policies/data.json");

/// Lightweight local policy evaluator.
fn evaluate_local(user: &PolicyUser, action: &str) -> bool {
    // Parse the compiled-in data.json
    let data: serde_json::Value = match serde_json::from_str(POLICY_DATA) {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Failed to parse embedded policy data: {}", e);
            return false;
        }
    };

    // Public endpoints bypass all checks.
    if let Some(public) = data["public_endpoints"].as_array()
        && public.iter().any(|p| p.as_str() == Some(action))
    {
        return true;
    }

    let roles_data = match data["roles"].as_object() {
        Some(r) => r,
        None => return false,
    };

    // Resolve effective roles (with inheritance).
    let mut effective_roles: Vec<&str> = Vec::new();
    for role in &user.roles {
        if let Some(role_def) = roles_data.get(role.as_str()) {
            if let Some(parent) = role_def["inherits"].as_str() {
                effective_roles.push(parent);
            } else {
                effective_roles.push(role.as_str());
            }
        }
    }

    // Collect permissions from effective roles.
    let mut has_permission = false;
    for role in &effective_roles {
        if let Some(role_def) = roles_data.get(*role)
            && let Some(perms) = role_def["permissions"].as_array()
            && perms.iter().any(|p| p.as_str() == Some(action))
        {
            has_permission = true;
            break;
        }
    }

    if !has_permission {
        return false;
    }

    // API key scope enforcement.
    if user.auth_source == "api_key" {
        let scope_ok = user.scopes.iter().any(|s| s == action) || {
            // Check wildcard scopes.
            if let Some((resource_type, _)) = action.split_once(':') {
                let wildcard = format!("{}:*", resource_type);
                user.scopes.iter().any(|s| s == &wildcard)
            } else {
                false
            }
        };
        if !scope_ok {
            return false;
        }
    }

    true
}

// ─── Public API ──────────────────────────────────────────────────

/// Check whether the user is allowed to perform `action`.
///
/// Returns `true` if allowed, `false` if denied.
pub async fn check_policy(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> bool {
    // Local mode: evaluate in-process without OPA sidecar.
    if local_mode() {
        let allowed = evaluate_local(user, action);
        if !allowed {
            tracing::info!(
                user_id = %user.user_id,
                action = %action,
                "Local policy denied"
            );
        }
        return allowed;
    }

    // OPA sidecar mode: HTTP POST.
    let url = format!("{}/{}", opa_url(), opa_path());
    let body = OpaInput {
        input: OpaInputBody {
            user: user.clone(),
            action: action.to_string(),
            resource: resource.cloned().unwrap_or_default(),
        },
    };

    match HTTP_CLIENT.post(&url).json(&body).send().await {
        Ok(resp) => match resp.json::<OpaResponse>().await {
            Ok(opa) => {
                let allowed = opa.result.unwrap_or(false);
                if !allowed {
                    tracing::info!(
                        user_id = %user.user_id,
                        action = %action,
                        "OPA denied"
                    );
                }
                allowed
            }
            Err(e) => {
                tracing::error!("Failed to parse OPA response: {}", e);
                fail_open()
            }
        },
        Err(e) => {
            tracing::error!("OPA request failed: {}", e);
            if fail_open() {
                tracing::warn!("OPA unreachable — failing open (ALLOW)");
                true
            } else {
                tracing::warn!("OPA unreachable — failing closed (DENY)");
                false
            }
        }
    }
}

/// Enforce policy — returns `Ok(())` if allowed, `Err(StatusCode)` if denied.
pub async fn enforce_policy(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> Result<(), axum::http::StatusCode> {
    if check_policy(user, action, resource).await {
        Ok(())
    } else {
        Err(axum::http::StatusCode::FORBIDDEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn opa_enabled_false_disables_sidecar() {
        assert!(sidecar_disabled(false, Some("false")));
        assert!(sidecar_disabled(false, Some("FALSE")));
        assert!(!sidecar_disabled(false, None));
    }

    #[test]
    fn admin_can_access_admin() {
        assert!(evaluate_local(&test_admin(), "admin:access"));
    }

    #[test]
    fn viewer_can_read_tasks() {
        assert!(evaluate_local(&test_viewer(), "tasks:read"));
    }

    #[test]
    fn viewer_cannot_write_tasks() {
        assert!(!evaluate_local(&test_viewer(), "tasks:write"));
    }

    #[test]
    fn viewer_cannot_access_admin() {
        assert!(!evaluate_local(&test_viewer(), "admin:access"));
    }

    #[test]
    fn api_key_in_scope_allowed() {
        assert!(evaluate_local(&test_api_key_user(), "tasks:read"));
    }

    #[test]
    fn api_key_out_of_scope_denied() {
        assert!(!evaluate_local(&test_api_key_user(), "admin:access"));
    }

    #[test]
    fn api_key_no_scope_for_workspaces() {
        assert!(!evaluate_local(&test_api_key_user(), "workspaces:read"));
    }

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

    #[test]
    fn a2a_admin_inherits_admin() {
        let user = test_user(&["a2a-admin"], &[], "keycloak");
        assert!(evaluate_local(&user, "admin:access"));
    }
}
