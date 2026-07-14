impl OpenAiCodexProvider {
    async fn complete_stream_with_openai_http_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let body = Self::build_http_responses_body(&request);
        Self::log_http_responses_request("openai-responses-http", &body);
        let response = self
            .client
            .post(format!("{OPENAI_API_URL}/responses"))
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenAI responses API")?;
        Self::validate_http_response(response, &request.model).await
    }
}
