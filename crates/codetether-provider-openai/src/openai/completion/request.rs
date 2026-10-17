//! request for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) async fn complete_request(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let response = self.send_completion(&request).await?;
        completion_response(response)
    }
}
