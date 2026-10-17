//! HTTP execution and response-envelope decoding.
use crate::response::wire::*;
use crate::*;
impl MoonshotProvider {
    pub(crate) async fn send_body(&self, body: &Value) -> Result<MoonshotResponse> {
        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Moonshot")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        super::error::check(status, &text)?;
        let response: MoonshotResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Moonshot response: {}",
            codetether_util::truncate_bytes_safe(&text, 200)
        ))?;

        Ok(response)
    }
}
