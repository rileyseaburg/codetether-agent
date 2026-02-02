//! Google AI provider implementation (stub)

use super::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct GoogleProvider {
    api_key: String,
}

impl GoogleProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self { api_key })
    }
}

#[async_trait]
impl Provider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "gemini-2.5-pro".to_string(),
                name: "Gemini 2.5 Pro".to_string(),
                provider: "google".to_string(),
                context_window: 1_000_000,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.25),
                output_cost_per_million: Some(10.0),
            },
            ModelInfo {
                id: "gemini-2.5-flash".to_string(),
                name: "Gemini 2.5 Flash".to_string(),
                provider: "google".to_string(),
                context_window: 1_000_000,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.15),
                output_cost_per_million: Some(0.6),
            },
        ])
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        // TODO: Implement using reqwest
        anyhow::bail!("Google provider not yet implemented")
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("Google provider not yet implemented")
    }
}
