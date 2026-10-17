//! Streaming request admission and HTTP errors.
use crate::*;
impl StepFunProvider {
    pub(crate) async fn send_stream(
        &self,
        chat_request: &ChatRequest,
    ) -> Result<reqwest::Response> {
        let response = self
            .client
            .post(format!("{}/chat/completions", STEPFUN_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            anyhow::bail!("StepFun API error ({}): {}", status, body);
        }

        Ok(response)
    }
}
