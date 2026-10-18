//! Native registry and runtime-policy hooks for shared schema selection.
#[cfg(test)]
#[path = "question_tests.rs"]
mod question_tests;
use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;
pub(super) fn active_tool_definitions(
    registry: &ToolRegistry,
    _selected_provider: &str,
) -> Vec<ToolDefinition> {
    codetether_tool_catalog::active_tool_definitions(
        registry.definitions(),
        codetether_question::runtime::available,
    )
}
pub(super) fn advertised_tools(
    model_supports_tools: bool,
    tools: &[ToolDefinition],
    provider: &str,
) -> Vec<ToolDefinition> {
    codetether_tool_catalog::advertised_tools(
        model_supports_tools,
        tools,
        provider,
        crate::runtime_policy::require_justification_in_ask_mode,
    )
}
