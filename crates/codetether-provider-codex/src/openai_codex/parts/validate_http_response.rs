impl OpenAiCodexProvider {
    async fn validate_http_response(
        response: reqwest::Response,
        model: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        codetether_codex_transport::validate_http_response(response, model).await
    }
}
