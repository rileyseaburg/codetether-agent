//! Evaluate embedded role and API-key scope policy locally.
use crate::PolicyUser;
static POLICY_DATA: &str = include_str!("../../../policies/data.json");

/// Lightweight local policy evaluator.
pub(crate) fn evaluate_local(user: &PolicyUser, action: &str) -> bool {
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

    if !crate::roles::has_permission(roles_data, user, action) {
        return false;
    }
    crate::scope::allowed(user, action)
}
