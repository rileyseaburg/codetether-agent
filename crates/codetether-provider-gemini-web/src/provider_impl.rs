//! Provider trait entrypoints and model listing.
use crate::*;

#[async_trait]
impl Provider for GeminiWebProvider {
    fn resolved_model_identity(&self, model: &str) -> String {
        Self::resolved_model(model)
    }
    fn name(&self) -> &str {
        "gemini-web"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(MODELS
            .iter()
            .map(|(id, _, label, ctx)| ModelInfo {
                id: id.to_string(),
                name: label.to_string(),
                provider: "gemini-web".to_string(),
                context_window: *ctx,
                max_output_tokens: Some(65_536),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            })
            .collect())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_response(request).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.stream_response(request).await
    }
}
