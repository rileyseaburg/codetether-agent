impl OpenAiCodexProvider {
    async fn connect_responses_ws_with_token(
        &self,
        token: &str,
        chatgpt_account_id: Option<&str>,
        backend: ResponsesWsBackend,
        session_id: &str,
    ) -> Result<OpenAiRealtimeConnection> {
        codetether_codex_transport::WebSocketTransport::new(
            &self.transport_health,
            &self.turn_states,
            &self.ws_pool,
        )
        .connect_responses_ws_with_token(token, chatgpt_account_id, backend, session_id)
        .await
    }
}
