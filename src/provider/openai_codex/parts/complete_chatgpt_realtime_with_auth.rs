impl OpenAiCodexProvider {
    async fn complete_chatgpt_realtime_with_auth(
        &self,
        request: CompletionRequest,
        access_token: String,
        account_id: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let first = self
            .complete_stream_with_realtime(
                request.clone(),
                access_token,
                Some(account_id.clone()),
                "chatgpt-codex-responses-ws",
                ResponsesWsBackend::ChatGptCodex,
                session_id,
            )
            .await;
        if !first.as_ref().is_err_and(stream_recovery::is_unauthorized) {
            return first;
        }
        let token = self.force_refresh_access_token().await?;
        let account = self
            .resolved_chatgpt_account_id(&token)
            .unwrap_or(account_id);
        self.complete_stream_with_realtime(
            request,
            token,
            Some(account),
            "chatgpt-codex-responses-ws",
            ResponsesWsBackend::ChatGptCodex,
            session_id,
        )
        .await
    }
}
