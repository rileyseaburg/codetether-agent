//! Tool-output summaries using application policy and session events.
use super::{Background, Notify, notify};
use crate::Application;
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;
use std::sync::Arc;

impl<H: Application, S: 'static> Background<H, S> {
    /// Return a cached summary, raw output, or bounded fallback while RLM runs.
    /// # Arguments
    /// `rendered`, `tool`, `input` and `call_id` describe the tool result.
    /// `session_id` and `messages` supply trace and transcript context.
    /// `model`, `provider` and `config` retain the caller's model policy.
    /// `notify` optionally receives progress as typed session events.
    /// # Returns
    /// Raw output, a ready cached summary, or a bounded pending placeholder.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, config: &codetether_rlm::RlmConfig) {
    /// let _output = codetether_application_providers::background::Background::<H, ()>::route_or_defer("output", "read", &serde_json::json!({}), "call", "session", &[], "model", provider, config, None);
    /// # }
    /// ```
    pub fn route_or_defer(
        rendered: &str,
        tool: &str,
        input: &serde_json::Value,
        call_id: &str,
        session_id: &str,
        messages: &[Message],
        model: &str,
        provider: Arc<dyn Provider>,
        config: &RlmConfig,
        notify: Option<Notify<S>>,
    ) -> String {
        codetether_rlm_background::route_or_defer(
            rendered,
            tool,
            input,
            call_id,
            session_id,
            messages,
            model,
            provider,
            config,
            notify::adapt(notify),
            Self::resolve_model,
        )
    }
}
