//! Deterministic tool-schema profiles for coding agents.
//!
//! The default registry remains complete. OpenAI Codex sessions advertise a
//! focused coding surface unless `CODETETHER_TOOL_PROFILE=full` is set.

#[path = "catalog.rs"]
mod catalog;
#[path = "selection.rs"]
mod selection;

use codetether_provider_core::ToolDefinition;

/// Returns whether an explicitly requested compact profile is active.
///
/// # Examples
///
/// ```rust
/// let _active = codetether_tool_core::profile::is_lean();
/// ```
pub fn is_lean() -> bool {
    selection::requested().is_coding()
}

/// Applies an explicit profile and deterministic ordering to tool definitions.
///
/// # Examples
///
/// ```rust
/// let definitions = Vec::new();
/// assert!(codetether_tool_core::profile::apply(definitions).is_empty());
/// ```
pub fn apply(mut definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    if selection::requested().is_mux_manager() {
        definitions = catalog::retain_mux_manager_tools(definitions);
    } else if is_lean() {
        definitions = catalog::retain_coding_tools(definitions);
    }
    catalog::sort(definitions)
}

/// Select and order advertised tool schemas for a provider.
///
/// # Arguments
/// * `definitions` - Registered tool definitions to filter and order.
/// * `provider` - Provider identity used when the requested profile is automatic.
/// # Returns
/// The selected definitions, sorted deterministically by tool name.
/// # Examples
/// ```rust
/// assert!(codetether_tool_core::profile::apply_for_provider(Vec::new(), "codex").is_empty());
/// ```
pub fn apply_for_provider(definitions: Vec<ToolDefinition>, provider: &str) -> Vec<ToolDefinition> {
    if selection::requested().is_mux_manager() {
        return catalog::sort(catalog::retain_mux_manager_tools(definitions));
    }
    if selection::use_discovery_profile(provider) {
        return catalog::sort(catalog::retain_discovery_tools(definitions));
    }
    if selection::use_coding_profile(provider) {
        return catalog::sort(catalog::retain_coding_tools(definitions));
    }
    catalog::sort(definitions)
}
