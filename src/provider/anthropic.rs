//! Anthropic provider implementation (stub)

use super::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct AnthropicProvider {
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self { api_key })
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                provider: "anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(64_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-opus-4-20250514".to_string(),
                name: "Claude Opus 4".to_string(),
                provider: "anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(32_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(15.0),
                output_cost_per_million: Some(75.0),
            },
        ])
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        // TODO: Implement using reqwest
        anyhow::bail!("Anthropic provider not yet implemented")
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("Anthropic provider not yet implemented")
    }
}
