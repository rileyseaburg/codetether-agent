impl OpenAiCodexProvider {
    async fn complete_stream_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let account_id = self.resolved_chatgpt_account_id(&access_token).context(
            "OpenAI Codex OAuth token is missing ChatGPT workspace/account ID. Re-run `codetether auth codex --device-code`.",
        )?;
        if Self::needs_chatgpt_http_transport(&request.model)
            || self.transport_health.requires_http()
        {
            return self
                .complete_stream_with_chatgpt_http_responses(request, access_token, account_id)
                .await;
        }
        match self
            .complete_stream_with_realtime(
                request.clone(),
                access_token.clone(),
                Some(account_id.clone()),
                "chatgpt-codex-responses-ws",
                ResponsesWsBackend::ChatGptCodex,
            )
            .await
        {
            Ok(stream) => Ok(stream_recovery::chatgpt(
                self.clone(),
                stream,
                request,
                access_token,
                account_id,
            )),
            Err(error) => {
                tracing::warn!(error = %error, "Codex transport unavailable; switching to HTTP");
                self.transport_health.mark_interrupted();
                self.complete_stream_with_chatgpt_http_responses(request, access_token, account_id)
                    .await
            }
        }
    }
}
