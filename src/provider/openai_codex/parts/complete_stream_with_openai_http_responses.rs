impl OpenAiCodexProvider {
    async fn complete_stream_with_openai_http_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let body = Self::build_http_responses_body(&request);
        Self::log_http_responses_request("openai-responses-http", &body);
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
        Self::validate_http_response(response, &request.model).await
    }
}
