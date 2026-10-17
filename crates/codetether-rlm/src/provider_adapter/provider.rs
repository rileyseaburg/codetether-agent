//! Adapt a shared provider handle to the RLM completion interface.

use super::{build_req, to_llm_resp};
use crate::traits::{LlmMessage, LlmProvider, LlmResponse, ToolDefinition};
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Wraps `Provider` as `LlmProvider`.
///
/// # Examples
/// ```no_run
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let adapter = codetether_rlm::provider_adapter::ProviderWrap(provider);
/// let _: &dyn codetether_rlm::traits::LlmProvider = &adapter;
/// # }
/// ```
pub struct ProviderWrap(
    /// Existing provider handle; completion calls are forwarded to it.
    pub Arc<dyn Provider>,
);

#[async_trait::async_trait]
impl LlmProvider for ProviderWrap {
    async fn complete(
        &self,
        messages: Vec<LlmMessage>,
        tools: Vec<ToolDefinition>,
        model: &str,
        temperature: Option<f32>,
    ) -> anyhow::Result<LlmResponse> {
        let req = build_req(messages, tools, model, temperature);
        let resp = self.0.complete(req).await?;
        Ok(to_llm_resp(resp))
    }
}
