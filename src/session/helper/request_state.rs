use std::path::Path;
use std::sync::Arc;

use crate::provider::{Provider, ToolDefinition};
use crate::tool::ToolRegistry;

use super::workspace_tools::registry_for_cwd;

#[path = "request_state/settings.rs"]
mod settings;
#[path = "request_state/tools.rs"]
mod tools;

pub(crate) struct ProviderStepState {
    pub tool_registry: Arc<ToolRegistry>,
    pub tool_definitions: Vec<ToolDefinition>,
    pub advertised_tool_definitions: Vec<ToolDefinition>,
    pub temperature: Option<f32>,
    pub model_supports_tools: bool,
    pub system_prompt: String,
}

pub(crate) fn build_provider_step_state(
    provider: Arc<dyn Provider>,
    selected_provider: &str,
    model: &str,
    cwd: &Path,
) -> ProviderStepState {
    let tool_registry = registry_for_cwd(provider, model, cwd);
    let tool_definitions = tools::active_tool_definitions(&tool_registry);
    let model_supports_tools = settings::model_supports_tools(selected_provider);
    let advertised_tool_definitions =
        tools::advertised_tools(model_supports_tools, &tool_definitions);
    let system_prompt = settings::system_prompt_for(
        selected_provider,
        model_supports_tools,
        &advertised_tool_definitions,
        cwd,
    );

    ProviderStepState {
        tool_registry,
        tool_definitions,
        advertised_tool_definitions,
        temperature: settings::temperature_for(model),
        model_supports_tools,
        system_prompt,
    }
}
