impl OpenAiCodexProvider {
    async fn validate_http_response(
        response: reqwest::Response,
        model: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(Self::format_openai_api_error(status, &body, model));
        }
        Ok(Self::drive_responses_http_stream(response))
    }
}
