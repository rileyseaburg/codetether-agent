//! Streaming request and HTTP admission.
use crate::*;
impl OpenRouterProvider {
    pub(crate) async fn start_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let body = request_body::build(
            &request,
            Self::convert_messages(&request.messages),
            Self::convert_tools(&request.tools),
            true,
        );

        tracing::debug!(
            provider = "openrouter",
            model = %request.model,
            message_count = request.messages.len(),
            body = %serde_json::to_string(&body).unwrap_or_default(), "Starting streaming request"
        );

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://codetether.run")
            .header("X-Title", "CodeTether Agent")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenRouter")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            if let Some(error_message) = Self::parse_error_body(&text) {
                anyhow::bail!(error_message);
            }
            anyhow::bail!("OpenRouter streaming error: {} {}", status, text);
        }

        Ok(stream::adapt(response.bytes_stream()))
    }
}
