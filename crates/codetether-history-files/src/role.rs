//! Stable role labels used in materialized turn paths.
use codetether_provider_core::Role;

/// Label a provider role for use in turn filenames.
///
/// # Arguments
/// * `role` - Provider message role.
/// # Returns
/// The stable role label; developer messages use the system label.
/// # Examples
/// ```rust
/// assert_eq!(codetether_history_files::role_label(&codetether_provider_core::Role::User),"user");
/// ```
pub fn role_label(role: &Role) -> &'static str {
    match role {
        Role::System | Role::Developer => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}
