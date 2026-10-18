//! Bind worker model selection to the application's cached provider policy.
use super::Background;
use crate::Application;
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmModelPurpose};
use std::sync::Arc;

impl<H: Application, S> Background<H, S> {
    /// Resolve a worker provider and model using application policy.
    /// # Arguments
    /// `provider` and `model` are fallbacks; `config` and `purpose` select policy.
    /// # Returns
    /// A boxed future producing the selected provider and model.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// let resolver: codetether_rlm_background::Resolver =
    ///     codetether_application_providers::background::Background::<H, ()>::resolve_model;
    /// # }
    /// ```
    pub fn resolve_model<'a>(
        provider: Arc<dyn Provider>,
        model: &'a str,
        config: &'a RlmConfig,
        purpose: RlmModelPurpose,
    ) -> codetether_rlm_background::Resolution<'a> {
        Box::pin(crate::model_resolution::resolve::<H>(
            provider, model, config, purpose,
        ))
    }
}
