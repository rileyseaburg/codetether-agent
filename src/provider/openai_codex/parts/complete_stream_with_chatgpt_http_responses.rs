impl OpenAiCodexProvider {
    async fn complete_stream_with_chatgpt_http_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
        account_id: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let body = Self::build_http_responses_body(&request);
        Self::log_http_responses_request("chatgpt-codex-responses-http", &body);
        let response = self
            .client
            .post(format!("{CHATGPT_CODEX_API_URL}/responses"))
            .header("Authorization", format!("Bearer {access_token}"))
            .header("chatgpt-account-id", account_id)
            .header("Content-Type", "application/json")
            .header("version", "0.144.0")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to ChatGPT Codex backend")?;
        Self::validate_http_response(response, &request.model).await
    }
}
