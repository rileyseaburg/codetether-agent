use crate::provider::ToolDefinition;
use crate::session::helper::bootstrap::list_tools_bootstrap_definition;
use crate::session::helper::runtime::is_interactive_tool;
use crate::tool::ToolRegistry;

fn is_dead_confirmation_tool(name: &str) -> bool {
    matches!(name, "confirm_edit" | "confirm_multiedit")
}

pub(super) fn active_tool_definitions(
    registry: &ToolRegistry,
    _selected_provider: &str,
) -> Vec<ToolDefinition> {
    registry
        .definitions()
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name))
        .filter(|tool| !is_dead_confirmation_tool(&tool.name))
        .collect()
}

pub(super) fn advertised_tools(
    model_supports_tools: bool,
    tools: &[ToolDefinition],
    provider: &str,
) -> Vec<ToolDefinition> {
    if !model_supports_tools {
        return Vec::new();
    }
    let mut advertised = crate::tool::profile::apply_for_provider(tools.to_vec(), provider);
    if advertised.len() < tools.len() {
        advertised.push(list_tools_bootstrap_definition());
    }
    advertised
}
