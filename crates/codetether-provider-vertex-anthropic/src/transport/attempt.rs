//! Retryable timeout and service-unavailable handling.
use crate::*;
impl VertexAnthropicProvider {
    pub(super) async fn attempt(
        &self,
        url: &str,
        access_token: &str,
        body: &Value,
        attempt: u32,
        last_err: &mut Option<String>,
    ) -> Result<Option<String>> {
        let response = match self.send_once(url, access_token, body).await {
            Ok(r) => r,
            Err(e) if e.is_timeout() && attempt + 1 < MAX_RETRIES => {
                tracing::warn!(error = %e, "Vertex Anthropic request timed out");
                *last_err = Some(format!("Request timed out: {e}"));
                return Ok(None);
            }
            Err(e) => anyhow::bail!("Failed to send request to Vertex AI Anthropic: {e}"),
        };

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Vertex AI Anthropic response")?;

        if status == reqwest::StatusCode::SERVICE_UNAVAILABLE && attempt + 1 < MAX_RETRIES {
            tracing::warn!(status = %status, body = %text, "Vertex Anthropic service unavailable, retrying");
            *last_err = Some(format!("503 Service Unavailable: {text}"));
            return Ok(None);
        }

        super::error::check(status, &text)?;
        Ok(Some(text))
    }
}
