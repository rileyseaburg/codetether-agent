impl OpenAiCodexProvider {
    async fn complete_stream_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let account_id = self.resolved_chatgpt_account_id(&access_token).context(
            "OpenAI Codex OAuth token is missing ChatGPT workspace/account ID. Re-run `codetether auth codex --device-code`.",
        )?;
        if Self::needs_chatgpt_http_transport(&request.model)
            || self.transport_health.requires_http(session_id)
        {
            return Ok(stream_recovery::chatgpt_http(
                self.clone(), request, session_id.to_string(),
            ));
        }
        match self
            .complete_chatgpt_realtime_with_auth(
                request.clone(), access_token, account_id, session_id,
            )
            .await
        {
            Ok(stream) => Ok(stream),
            Err(error) if stream_recovery::is_upgrade_required(&error) => {
                tracing::warn!(error = %error, "Codex transport unavailable; switching to HTTP");
                self.transport_health.mark_interrupted(session_id);
                Ok(stream_recovery::chatgpt_http(
                    self.clone(), request, session_id.to_string(),
                ))
            }
            Err(error) => Err(stream_recovery::tag_stream_open(error)),
        }
    }
}
