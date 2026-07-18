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
        Self::log_responses_ws_request(&body, backend);

        let connection = self
            .pooled_ws_connection(
                &access_token,
                chatgpt_account_id.as_deref(),
                ws_backend,
                session_id,
            )
            .await?;

        Ok(ws_stream::drive(
            connection,
            body,
            session_id.to_string(),
            self.transport_health.clone(),
            self.ws_pool.clone(),
        ))
    }
}
