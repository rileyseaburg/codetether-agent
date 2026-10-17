impl OpenAiCodexProvider {
    async fn complete_stream_with_openai_http_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let body = Self::build_http_responses_body(&request);
        Self::log_http_responses_request("openai-responses-http", &body);
        let response =
            codetether_codex_transport::HttpTransport::new(&self.client, &self.turn_states)
                .send_openai_response(&body, &api_key, session_id)
                .await?;
        Self::validate_http_response(response, &request.model).await
    }
}
