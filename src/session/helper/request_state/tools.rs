use crate::provider::ToolDefinition;
use crate::session::helper::bootstrap::list_tools_bootstrap_definition;
use crate::session::helper::runtime::is_interactive_tool;
use crate::tool::ToolRegistry;

fn is_dead_confirmation_tool(name: &str) -> bool {
    matches!(name, "confirm_edit" | "confirm_multiedit")
}

pub(super) fn active_tool_definitions(
    registry: &ToolRegistry,
    selected_provider: &str,
) -> Vec<ToolDefinition> {
    let definitions = registry
        .definitions()
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name))
        .filter(|tool| !is_dead_confirmation_tool(&tool.name))
        .collect();
    crate::tool::profile::apply_for_provider(definitions, selected_provider)
}

pub(super) fn advertised_tools(
    model_supports_tools: bool,
    tools: &[ToolDefinition],
) -> Vec<ToolDefinition> {
    if model_supports_tools {
        tools.to_vec()
    } else {
        vec![list_tools_bootstrap_definition()]
    }
}
