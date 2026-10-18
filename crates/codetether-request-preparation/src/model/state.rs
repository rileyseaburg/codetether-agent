//! Active provider and tool settings.
use crate::ProviderStepState;
use codetether_provider_core::{Provider, ToolDefinition};
use std::sync::Arc;

/// Provider, model, and tool settings active for the current step.
/// # Examples
/// ```no_run
/// # use std::sync::Arc;
/// # use codetether_provider_core::Provider;
/// # use codetether_request_preparation::{ProviderStepState, model::ModelState};
/// # fn configure<R>(provider: Arc<dyn Provider>, step: ProviderStepState<R>) {
/// let state = ModelState::new("provider".into(), "model".into(), provider, step);
/// assert_eq!(state.model_id, "model");
/// # }
/// ```
pub struct ModelState<R> {
    /// Registry name of the selected provider.
    pub provider_name: String,
    /// Provider-specific model identifier.
    pub model_id: String,
    /// Provider client used for completion requests.
    pub provider: Arc<dyn Provider>,
    /// Rebuildable provider-step configuration.
    pub step: ProviderStepState<R>,
    /// Executable tools available in the workspace.
    pub registry: Arc<R>,
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
