//! Buffered HTTP request and response-envelope decoding.
use crate::*;
impl OpenRouterProvider {
    pub(crate) async fn send_body(&self, body: &Value) -> Result<OpenRouterResponse> {
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
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if let Some(error_message) = Self::parse_error_body(&text) {
            anyhow::bail!(error_message);
        }

        if !status.is_success() {
            anyhow::bail!("OpenRouter API error: {} {}", status, text);
        }

        tracing::debug!(
            "OpenRouter response: {}",
            codetether_util::truncate_bytes_safe(&text, 500)
        );

        let response: OpenRouterResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse response: {}",
            codetether_util::truncate_bytes_safe(&text, 200)
        ))?;

        Ok(response)
    }
}
