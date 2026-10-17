//! Send a public OpenAI Responses request with routing headers.
use super::*;
impl HttpTransport<'_> {
    /// Send one public OpenAI HTTP request and capture successful routing state.
    ///
    /// # Errors
    /// Returns the HTTP send error with the existing backend context.
    pub async fn send_openai_response(
        &self,
        body: &Value,
        api_key: &str,
        session_id: &str,
    ) -> Result<reqwest::Response> {
        let mut request_builder = self
            .client
            .post(format!("{OPENAI_API_URL}/responses"))
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&body);
        let turn_state = self.turn_states.current(session_id);
        if let Some(value) = turn_state.get() {
            request_builder = request_builder.header(X_CODEX_TURN_STATE_HEADER, value);
        }
        let response = request_builder
            .send()
            .await
            .context("Failed to send streaming request to OpenAI responses API")?;
        if response.status().is_success()
            && let Some(value) = response.headers().get(X_CODEX_TURN_STATE_HEADER)
            && let Ok(value) = value.to_str()
        {
            self.turn_states.capture(session_id, value);
        }
        Ok(response)
    }
}
