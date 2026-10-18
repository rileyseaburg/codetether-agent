//! Registration-group dispatch and provider-first scalar selection.
use super::{Host, ToolKind, groups};

/// Application effects retained outside ordered catalog dispatch.
/// # Implementors
/// Native registries supply command-session setup, feature availability, and
/// concrete tool construction. The shared dispatcher expands catalog groups.
pub trait DispatchHost: Host {
    /// Register the application command-session group.
    fn register_commands(&mut self);
    /// Whether scripting tools were compiled into this application.
    fn scripting_enabled(&self) -> bool;
    /// Construct/register a provider-backed tool, returning whether handled.
    fn register_provider(&mut self, kind: ToolKind) -> bool;
    /// Construct/register a scalar tool after provider dispatch declines it.
    fn register_scalar(&mut self, kind: ToolKind);
}

/// Expand a catalog group or register a provider-backed or scalar tool.
/// # Arguments
/// * `host` — Native construction and registration effects.
/// * `kind` — Requested tool or registration group.
/// # Returns
/// Completes after registration in the existing group and fallback order.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_tool_registry::catalog::dispatch::DispatchHost) {
/// codetether_tool_registry::catalog::dispatch::register(
///     host, codetether_tool_registry::catalog::ToolKind::SearchGroup,
/// );
/// # }
/// ```
pub fn register(host: &mut impl DispatchHost, kind: ToolKind) {
    match kind {
        ToolKind::SearchGroup => groups::search(host),
        ToolKind::CommandGroup => host.register_commands(),
        ToolKind::GoalGroup => groups::goals(host),
        ToolKind::MediaGroup => groups::media(host),
        ToolKind::CollaborationGroup => groups::collaboration(host),
        ToolKind::ScriptingGroup => {
            if host.scripting_enabled() {
                groups::scripting(host);
            }
        }
        ToolKind::CompatibilityGroup => groups::compatibility(host),
        _ => {
            if host.register_provider(kind) {
                return;
            }
            host.register_scalar(kind);
        }
    }
}
