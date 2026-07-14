//! Provider and tool configuration active in the shared loop.

use super::super::request_state::ProviderStepState;
use crate::provider::{Provider, ToolDefinition};
use crate::tool::ToolRegistry;
use std::sync::Arc;

/// Provider, model, and tool settings active for the current step.
pub(crate) struct ModelState {
    /// Registry name of the selected provider.
    pub provider_name: String,
    /// Provider-specific model identifier.
    pub model_id: String,
    /// Provider client used for completion requests.
    pub provider: Arc<dyn Provider>,
    /// Rebuildable provider-step configuration.
    pub step: ProviderStepState,
    /// Executable tools available in the workspace.
    pub registry: Arc<ToolRegistry>,
    /// Complete definitions for executable tools.
    pub tools: Vec<ToolDefinition>,
    /// Tool definitions advertised to the model.
    pub advertised: Vec<ToolDefinition>,
    /// Sampling temperature required by the selected model.
    pub temperature: Option<f32>,
    /// Whether native tool calls are supported.
    pub supports_tools: bool,
    /// System prompt assembled for this provider and workspace.
    pub system_prompt: String,
}

impl ModelState {
    /// Builds model state from a freshly derived provider-step configuration.
    pub fn new(
        provider_name: String,
        model_id: String,
        provider: Arc<dyn Provider>,
        step: ProviderStepState,
    ) -> Self {
        Self {
            provider_name,
            model_id,
            provider,
            registry: step.tool_registry.clone(),
            tools: step.tool_definitions.clone(),
            advertised: step.advertised_tool_definitions.clone(),
            temperature: step.temperature,
            supports_tools: step.model_supports_tools,
            system_prompt: step.system_prompt.clone(),
            step,
        }
    }
}
