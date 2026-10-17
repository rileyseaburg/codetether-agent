//! Buffered completion orchestration.
use crate::*;
impl CopilotProvider {
    pub(crate) async fn complete_inner(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let (body, is_agent, has_vision) = Self::body(&request);

        let response = self.send(&body, is_agent, has_vision).await?;

        let text = read::read(response).await?;

        let response: response::CopilotResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Copilot response: {}",
            codetether_util::truncate_bytes_safe(&text, 200)
        ))?;
        response::to_completion_response(response)
    }
}
mod read;
mod send;
