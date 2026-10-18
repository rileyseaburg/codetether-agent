//! Provider-step state shared across prompt-loop stages.

use std::sync::Arc;

use codetether_provider_core::ToolDefinition;

/// Tool and provider settings derived for one prompt-loop step.
/// # Examples
/// ```
/// let state = codetether_request_preparation::ProviderStepState {
///     tool_registry: std::sync::Arc::new(()), tool_definitions: vec![],
///     advertised_tool_definitions: vec![], temperature: Some(0.7),
///     model_supports_tools: true, system_prompt: "base".into(),
/// };
/// assert_eq!(state.system_prompt, "base");
/// ```
pub struct ProviderStepState<R> {
    /// Workspace-scoped executable tool registry.
    pub tool_registry: Arc<R>,
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
