impl OpenAiCodexProvider {
    async fn complete_stream_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        if self.transport_health.requires_http() {
            return self
                .complete_stream_with_openai_http_responses(request, api_key)
                .await;
        }
        match self
            .complete_stream_with_realtime(
                request.clone(),
                api_key.clone(),
                None,
                "openai-responses-ws",
                ResponsesWsBackend::OpenAi,
            )
            .await
        {
            Ok(stream) => Ok(stream_recovery::openai(
                self.clone(),
                stream,
                request,
                api_key,
            )),
            Err(error) => {
                tracing::warn!(error = %error, "Codex transport unavailable; switching to HTTP");
                self.transport_health.mark_interrupted();
                self.complete_stream_with_openai_http_responses(request, api_key)
                    .await
            }
        }
    }
}
