//! Borrowed mutable provider-step fields.
use crate::ProviderStepState;
use codetether_provider_core::{Provider, ToolDefinition};
use std::sync::Arc;

/// All mutable step-loop state that may need restoring at each step start.
/// # Examples
/// ```no_run
/// # use std::sync::Arc;
/// # use codetether_provider_core::Provider;
/// # use codetether_request_preparation::{ProviderStepState, step_model::StepVars};
/// # fn borrow(provider: &mut Arc<dyn Provider>, state: &mut ProviderStepState<()>) {
/// let (mut name, mut model, mut prompt) = (String::new(), String::new(), String::new());
/// let (mut tools, mut advertised) = (Vec::new(), Vec::new());
/// let (mut registry, mut temperature, mut supports, mut session) = (Arc::new(()), None, false, ());
/// let vars = StepVars {
///     selected_provider: &mut name, model: &mut model, provider, provider_state: state,
///     tool_registry: &mut registry, tool_definitions: &mut tools,
///     advertised_tool_definitions: &mut advertised, temperature: &mut temperature,
///     model_supports_tools: &mut supports, system_prompt: &mut prompt, session: &mut session,
/// };
/// assert!(!*vars.model_supports_tools);
/// # }
/// ```
pub struct StepVars<'a, R, S> {
    /// Registry name of the active provider.
    pub selected_provider: &'a mut String,
    /// Active model identifier.
    pub model: &'a mut String,
    /// Client used for provider requests.
    pub provider: &'a mut Arc<dyn Provider>,
    /// Rebuildable settings derived for the active target.
    pub provider_state: &'a mut ProviderStepState<R>,
    /// Active executable tool registry.
    pub tool_registry: &'a mut Arc<R>,
    /// Complete tool definitions.
    pub tool_definitions: &'a mut Vec<ToolDefinition>,
    /// Tool definitions advertised to the provider.
    pub advertised_tool_definitions: &'a mut Vec<ToolDefinition>,
    /// Model-specific sampling temperature.
    pub temperature: &'a mut Option<f32>,
    /// Whether the active model accepts native tool calls.
    pub model_supports_tools: &'a mut bool,
    /// Prepared system instructions.
    pub system_prompt: &'a mut String,
    /// Native session used for metadata and preparation.
    pub session: &'a mut S,
}
