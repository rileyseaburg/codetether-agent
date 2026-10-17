//! Buffered request preparation and interpretation.
use crate::*;
impl OpenRouterProvider {
    pub(crate) async fn complete_buffered(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let body = request_body::build(
            &request,
            Self::convert_messages(&request.messages),
            Self::convert_tools(&request.tools),
            false,
        );

        tracing::debug!(
            "OpenRouter request: {}",
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );

        crate::response::parse(self.send_body(&body).await?)
    }
}
