//! Streaming request and non-success response handling.
use crate::*;
impl VertexAnthropicProvider {
    pub(super) async fn send_stream(
        &self,
        url: &str,
        access_token: &str,
        body: &Value,
    ) -> Result<reqwest::Response> {
        let response = self
            .client
            .post(url)
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Vertex AI Anthropic")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<AnthropicError>(&text) {
                anyhow::bail!(
                    "Vertex AI Anthropic API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Vertex AI Anthropic streaming error: {} {}", status, text);
        }

        Ok(response)
    }
}
