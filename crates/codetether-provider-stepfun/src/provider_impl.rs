//! Provider contract and request entry points.
use crate::*;
use async_trait::async_trait;
#[async_trait]
impl Provider for StepFunProvider {
    fn name(&self) -> &str {
        "stepfun"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(crate::catalog::models())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        tracing::debug!(
            provider = "stepfun",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting completion request"
        );

        // Validate API key before making request
        self.validate_api_key()?;

        let chat_request = self.build_request(&request, false);
        // Debug: log the request being sent
        if let Ok(json_str) = serde_json::to_string_pretty(&chat_request) {
            tracing::debug!("StepFun request: {}", json_str);
        }

        crate::response::parse(self.send_request(&chat_request).await?)
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        tracing::debug!(
            provider = "stepfun",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting streaming completion request"
        );

        self.validate_api_key()?;

        let chat_request = self.build_request(&request, true);
        Ok(crate::streaming::decode(
            self.send_stream(&chat_request).await?,
        ))
    }
}
