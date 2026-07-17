impl OpenAiCodexProvider {
    async fn pooled_ws_connection(
        &self,
        access_token: &str,
        chatgpt_account_id: Option<&str>,
        backend: ResponsesWsBackend,
    ) -> Result<OpenAiRealtimeConnection> {
        if let Some(connection) = self.ws_pool.take().await {
            tracing::debug!("Reusing Codex responses websocket");
            return Ok(connection);
        }
        self.connect_responses_ws_with_token(access_token, chatgpt_account_id, backend)
            .await
    }
}
