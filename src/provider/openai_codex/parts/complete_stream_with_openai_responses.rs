impl OpenAiCodexProvider {
    async fn complete_stream_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        if self.transport_health.requires_http(session_id) {
            return Ok(stream_recovery::openai_http(
                self.clone(), request, api_key, session_id.to_string(),
            ));
        }
        match self
            .complete_stream_with_realtime(
                request.clone(),
                api_key.clone(),
                None,
                "openai-responses-ws",
                ResponsesWsBackend::OpenAi,
                session_id,
            )
            .await
        {
            Ok(stream) => Ok(stream),
            Err(error) if stream_recovery::is_upgrade_required(&error) => {
                tracing::warn!(error = %error, "Codex transport unavailable; switching to HTTP");
                self.transport_health.mark_interrupted(session_id);
                Ok(stream_recovery::openai_http(
                    self.clone(), request, api_key, session_id.to_string(),
                ))
            }
            Err(error) => Err(stream_recovery::tag_stream_open(error)),
        }
    }
}
