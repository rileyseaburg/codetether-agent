//! Stable human-readable roles for context transcripts.
use codetether_provider_core::Role;

/// Return the transcript label for a message role.
///
/// # Arguments
/// * `role` - Provider role to display.
///
/// # Returns
/// The stable label; developer messages use the system label.
///
/// # Examples
/// ```
/// use codetether_context_policy::transcript::role_label;
/// use codetether_provider_core::Role;
/// assert_eq!(role_label(Role::Developer), "System");
/// ```
pub fn role_label(role: Role) -> &'static str {
    match role {
        Role::System | Role::Developer => "System",
        Role::User => "User",
        Role::Assistant => "Assistant",
        Role::Tool => "Tool",
    }
}
