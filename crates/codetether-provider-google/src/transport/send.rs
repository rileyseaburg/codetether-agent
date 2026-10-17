//! Buffered completion request execution.
use crate::response::wire::*;
use crate::*;
impl GoogleProvider {
    pub(crate) async fn send_body(&self, body: &Value) -> Result<ChatCompletion> {
        // Google AI Studio OpenAI-compatible endpoint uses Bearer token auth
        let url = format!("{}/chat/completions", GOOGLE_OPENAI_BASE);
        let response = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Google Gemini")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Google Gemini response")?;

        super::error::check(status, &text)?;
        let completion: ChatCompletion = serde_json::from_str(&text).context(format!(
            "Failed to parse Google Gemini response: {}",
            codetether_util::truncate_bytes_safe(&text, 200)
        ))?;

        Ok(completion)
    }
}
