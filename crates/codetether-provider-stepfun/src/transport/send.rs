//! Buffered HTTP request and envelope decoding.
use crate::*;
impl StepFunProvider {
    pub(crate) async fn send_request(&self, chat_request: &ChatRequest) -> Result<ChatResponse> {
        let response = self
            .client
            .post(format!("{}/chat/completions", STEPFUN_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        super::error::check(status, &body)?;
        let chat_response: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {} - Body: {}", e, body))?;

        Ok(chat_response)
    }
}
