//! Provider adapter: main-crate `Provider` → crate `LlmProvider`.

use super::adapter_convert::{build_req, to_llm_resp};
use crate::provider::Provider;
use codetether_rlm::traits::{LlmMessage, LlmProvider, LlmResponse, ToolDefinition};
use std::sync::Arc;

/// Wraps `Provider` as `LlmProvider`.
pub(super) struct ProviderWrap(pub(crate) Arc<dyn Provider>);

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
