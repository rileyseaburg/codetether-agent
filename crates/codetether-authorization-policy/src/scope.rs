//! Enforce exact and resource-wildcard API-key scopes.
use crate::PolicyUser;
pub(crate) fn allowed(user: &PolicyUser, action: &str) -> bool {
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
