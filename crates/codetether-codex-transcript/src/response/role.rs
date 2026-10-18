//! Convert supported archive roles into provider message roles.
use codetether_provider_core::Role;

pub(super) fn map_role(role: &str) -> Option<Role> {
    match role {
        "user" => Some(Role::User),
        "assistant" => Some(Role::Assistant),
        "developer" => Some(Role::Developer),
        _ => None,
    }
}
