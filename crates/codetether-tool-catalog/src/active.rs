//! Filter executable schemas by frontend availability and confirmation lifecycle.
use codetether_prompt_policy::is_interactive_tool;
use codetether_provider_core::ToolDefinition;

fn is_dead_confirmation_tool(name: &str) -> bool {
    matches!(name, "confirm_edit" | "confirm_multiedit")
}

/// Keep usable tools while excluding obsolete explicit-confirmation schemas.
/// # Arguments
/// `definitions` is the registry catalog; `interactive_available` reports
/// whether a frontend can currently handle an interactive tool.
/// # Returns
/// The filtered catalog in its original order.
/// # Examples
/// ```
/// assert!(codetether_tool_catalog::active_tool_definitions(vec![], || false).is_empty());
/// ```
pub fn active_tool_definitions(
    definitions: Vec<ToolDefinition>,
    interactive_available: impl Fn() -> bool,
) -> Vec<ToolDefinition> {
    definitions
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name) || interactive_available())
        .filter(|tool| !is_dead_confirmation_tool(&tool.name))
        .collect()
}
