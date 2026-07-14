impl OpenAiCodexProvider {
    async fn complete_stream_with_realtime(
        &self,
        request: CompletionRequest,
        access_token: String,
        chatgpt_account_id: Option<String>,
        backend: &'static str,
        ws_backend: ResponsesWsBackend,
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
        tracing::info!(
            backend = backend,
            instructions_len = body
                .get("instructions")
                .and_then(|v| v.as_str())
                .map(str::len)
                .unwrap_or(0),
            input_items = body
                .get("input")
                .and_then(|v| v.as_array())
                .map(Vec::len)
                .unwrap_or(0),
            has_tools = body
                .get("tools")
                .and_then(|v| v.as_array())
                .is_some_and(|tools| !tools.is_empty()),
            "Sending responses websocket request"
        );

        let connection = self
            .connect_responses_ws_with_token(
                &access_token,
                chatgpt_account_id.as_deref(),
                ws_backend,
            )
            .await?;

        Ok(ws_stream::drive(
            connection,
            body,
            self.transport_health.clone(),
        ))
    }
}
