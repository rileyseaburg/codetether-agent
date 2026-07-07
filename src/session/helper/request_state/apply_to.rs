//! `ProviderStepState::apply_to` — sync derived loop variables from state.

use std::sync::Arc;

use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;

use super::ProviderStepState;

impl ProviderStepState {
    /// Write all derived fields into the caller's mutable step-loop variables.
    pub(crate) fn apply_to(
        &self,
        tool_registry: &mut Arc<ToolRegistry>,
        tool_definitions: &mut Vec<ToolDefinition>,
        temperature: &mut Option<f32>,
        model_supports_tools: &mut bool,
        advertised_tool_definitions: &mut Vec<ToolDefinition>,
        system_prompt: &mut String,
    ) {
        *tool_registry = self.tool_registry.clone();
        *tool_definitions = self.tool_definitions.clone();
        *temperature = self.temperature;
        *model_supports_tools = self.model_supports_tools;
        *advertised_tool_definitions = self.advertised_tool_definitions.clone();
        *system_prompt = self.system_prompt.clone();
    }
}
