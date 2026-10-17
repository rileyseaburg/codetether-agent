impl OpenAiCodexProvider {
    async fn complete_stream_with_realtime(
        &self,
        request: CompletionRequest,
        access_token: String,
        chatgpt_account_id: Option<String>,
        backend: &'static str,
        ws_backend: ResponsesWsBackend,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let (model, reasoning_effort, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
        let body = Self::build_responses_ws_create_event_for_backend(
            &request,
            &model,
            reasoning_effort,
            service_tier,
            ws_backend,
        );
        codetether_codex_transport::WebSocketTransport::new(
            &self.transport_health,
            &self.turn_states,
            &self.ws_pool,
        )
        .start_stream(
            body,
            access_token,
            chatgpt_account_id,
            backend,
            ws_backend,
            session_id,
        )
        .await
    }
}
