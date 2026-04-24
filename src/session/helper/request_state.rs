use std::path::Path;
use std::sync::Arc;

use crate::provider::{Provider, ToolDefinition};
use crate::tool::ToolRegistry;

use super::bootstrap::{inject_tool_prompt, list_tools_bootstrap_definition};
use super::provider::{prefers_temperature_one, temperature_is_deprecated};
use super::runtime::{is_interactive_tool, is_local_cuda_provider, local_cuda_light_system_prompt};

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
    let tool_registry = ToolRegistry::with_provider_arc(provider, model.to_string());
    let tool_definitions: Vec<_> = tool_registry
        .definitions()
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name))
        .collect();

    let temperature = if temperature_is_deprecated(model) {
        None
    } else if prefers_temperature_one(model) {
        Some(1.0)
    } else {
        Some(0.7)
    };

    let model_supports_tools = !matches!(
        selected_provider,
        "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
    );
    let advertised_tool_definitions = if model_supports_tools {
        tool_definitions.clone()
    } else {
        vec![list_tools_bootstrap_definition()]
    };

    let system_prompt = if is_local_cuda_provider(selected_provider) {
        local_cuda_light_system_prompt()
    } else {
        crate::agent::builtin::build_system_prompt(cwd)
    };
    let system_prompt = if !model_supports_tools && !advertised_tool_definitions.is_empty() {
        inject_tool_prompt(&system_prompt, &advertised_tool_definitions)
    } else {
        system_prompt
    };

    ProviderStepState {
        tool_registry,
        tool_definitions,
        advertised_tool_definitions,
        temperature,
        model_supports_tools,
        system_prompt,
    }
}
