//! Resolve effective roles and inspect their permitted actions.
use crate::PolicyUser;
pub(crate) fn has_permission(
    roles_data: &serde_json::Map<String, serde_json::Value>,
    user: &PolicyUser,
    action: &str,
) -> bool {
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

    has_permission
}
