//! Anthropic provider implementation (stub)

use super::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;

pub struct AnthropicProvider {
    api_key: String,
}

impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .finish()
    }
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "anthropic",
            api_key_len = api_key.len(),
            "Creating Anthropic provider"
        );
        Ok(Self { api_key })
    }

    /// Validate that the API key is non-empty
    fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Anthropic API key is empty");
        }
        if self.api_key.len() < 10 {
            tracing::warn!(provider = "anthropic", "API key seems unusually short");
        }
        Ok(())
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        tracing::debug!(provider = "anthropic", "Listing available models");
        self.validate_api_key()?;

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

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        tracing::debug!(
            provider = "anthropic",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting completion request"
        );

        // Validate API key before making request
        self.validate_api_key()?;

        // TODO: Implement using reqwest
        anyhow::bail!("Anthropic provider not yet implemented")
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        tracing::debug!(
            provider = "anthropic",
            model = %request.model,
            message_count = request.messages.len(),
            "Starting streaming completion request"
        );

        self.validate_api_key()?;
        anyhow::bail!("Anthropic provider not yet implemented")
    }
}
