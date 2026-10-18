use super::ModelState;
use crate::ProviderStepState;
use codetether_provider_core::Provider;
use std::sync::Arc;

impl<R> ModelState<R> {
    /// Builds model state from a freshly derived provider-step configuration.
    /// # Arguments
    /// `provider_name` and `model_id` identify the selected target; `provider`
    /// is its client; `step` contains derived tool and prompt settings.
    /// # Returns
    /// Active state with cloned settings and the retained derivation snapshot.
    /// # Examples
    /// ```no_run
    /// # fn configure<R>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, step: codetether_request_preparation::ProviderStepState<R>) {
    /// let state = codetether_request_preparation::model::ModelState::new(
    ///     "provider".into(), "model".into(), provider, step,
    /// );
    /// assert_eq!(state.provider_name, "provider");
    /// # }
    /// ```
    pub fn new(
        provider_name: String,
        model_id: String,
        provider: Arc<dyn Provider>,
        step: ProviderStepState<R>,
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
