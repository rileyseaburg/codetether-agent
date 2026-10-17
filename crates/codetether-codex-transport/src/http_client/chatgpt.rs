//! Send a ChatGPT Responses request with account and routing headers.
use super::*;
impl HttpTransport<'_> {
    /// Send one ChatGPT HTTP request using the supplied payload and credentials.
    ///
    /// # Errors
    /// Returns the HTTP send error with the existing backend context.
    pub async fn send_chatgpt_http_response(
        &self,
        body: &Value,
        access_token: &str,
        account_id: &str,
        session_id: &str,
    ) -> Result<reqwest::Response> {
        let mut builder = self
            .client
            .post(format!("{CHATGPT_CODEX_API_URL}/responses"))
            .header("Authorization", format!("Bearer {access_token}"))
            .header("chatgpt-account-id", account_id)
            .header("Content-Type", "application/json")
            .header("version", CHATGPT_CODEX_CLIENT_VERSION)
            .json(&body);
        if let Some(value) = self.turn_states.current(session_id).get() {
            builder = builder.header(X_CODEX_TURN_STATE_HEADER, value);
        }
        let response = builder
            .send()
            .await
            .context("Failed to send streaming request to ChatGPT Codex backend")?;
        if response.status().is_success()
            && let Some(value) = response.headers().get(X_CODEX_TURN_STATE_HEADER)
            && let Ok(value) = value.to_str()
        {
            self.turn_states.capture(session_id, value);
        }
        Ok(response)
    }
}
