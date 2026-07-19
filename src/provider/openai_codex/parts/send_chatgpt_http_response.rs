impl OpenAiCodexProvider {
    async fn send_chatgpt_http_response(
        &self,
        request: &CompletionRequest,
        access_token: &str,
        account_id: &str,
        session_id: &str,
    ) -> Result<reqwest::Response> {
        let body = Self::build_http_responses_body(request);
        Self::log_http_responses_request("chatgpt-codex-responses-http", &body);
        let mut builder = self
            .client
            .post(format!("{CHATGPT_CODEX_API_URL}/responses"))
            .header("Authorization", format!("Bearer {access_token}"))
            .header("chatgpt-account-id", account_id)
            .header("Content-Type", "application/json")
            .header("version", "0.144.0")
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
