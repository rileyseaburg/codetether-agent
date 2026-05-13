use crate::provider::ToolDefinition;
use crate::session::helper::bootstrap::list_tools_bootstrap_definition;
use crate::session::helper::runtime::is_interactive_tool;
use crate::tool::ToolRegistry;

pub(super) fn active_tool_definitions(registry: &ToolRegistry) -> Vec<ToolDefinition> {
    registry
        .definitions()
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name))
        .collect()
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
