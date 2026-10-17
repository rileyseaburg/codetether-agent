//! Provider dispatch and structured-streaming capability.
use crate::*;
use async_trait::async_trait;
#[async_trait]
impl Provider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
    }

    fn supports_structured_streaming(&self) -> bool {
        false
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;
        Ok(crate::catalog::models())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        tracing::debug!(
            provider = "google",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting Google Gemini completion request"
        );

        self.validate_api_key()?;

        let body = Self::build_body(&request);
        tracing::debug!("Google Gemini request to model {}", request.model);

        crate::response::parse(self.send_body(&body).await?)
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
