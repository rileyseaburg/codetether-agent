//! Provider-step state shared across prompt-loop stages.

use std::sync::Arc;

use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;

/// Tool and provider settings derived for one prompt-loop step.
pub(crate) struct ProviderStepState {
    /// Workspace-scoped executable tool registry.
    pub tool_registry: Arc<ToolRegistry>,
    /// Complete active tool definitions.
    pub tool_definitions: Vec<ToolDefinition>,
    /// Definitions advertised to the selected provider.
    pub advertised_tool_definitions: Vec<ToolDefinition>,
    /// Effective sampling temperature.
    pub temperature: Option<f32>,
    /// Whether the selected model supports native tools.
    pub model_supports_tools: bool,
    /// Fully assembled system prompt.
    pub system_prompt: String,
}
