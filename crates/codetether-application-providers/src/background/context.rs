//! Context summaries using application model policy.
use super::Background;
use crate::Application;
use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use std::sync::Arc;

impl<H: Application, S> Background<H, S> {
    /// Return a cached context summary or schedule a worker.
    /// # Arguments
    /// `content` and `reason` identify the summary; `session_id` identifies its trace.
    /// `model`, `provider` and `config` supply the caller's model and RLM settings.
    /// # Returns
    /// A completed summary, or `None` while background work is pending.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, config: &codetether_rlm::RlmConfig) {
    /// let _summary = codetether_application_providers::background::Background::<H, ()>::context_summary("text", "budget", "session", "model", provider, config);
    /// # }
    /// ```
    pub fn context_summary(
        content: &str,
        reason: &str,
        session_id: &str,
        model: &str,
        provider: Arc<dyn Provider>,
        config: &RlmConfig,
    ) -> Option<String> {
        codetether_rlm_background::context_summary(
            content,
            reason,
            session_id,
            model,
            provider,
            config,
            Self::resolve_model,
        )
    }
}
