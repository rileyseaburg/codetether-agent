#[async_trait]
impl Provider for OpenAiCodexProvider {
    fn name(&self) -> &str {
        "openai-codex"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(self.available_models())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_in_session(request, transport_health::UNSCOPED)
            .await
    }

    async fn complete_scoped(
        &self,
        request: CompletionRequest,
        session_id: &str,
    ) -> Result<CompletionResponse> {
        self.complete_in_session(request, session_id).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.complete_stream_in_session(request, transport_health::UNSCOPED)
            .await
    }

    async fn complete_stream_scoped(
        &self,
        request: CompletionRequest,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.complete_stream_in_session(request, session_id).await
    }

    fn begin_stream_recovery(&self, session_id: &str) {
        self.turn_states.begin(session_id);
    }

    fn try_stream_fallback(&self, request: &CompletionRequest, session_id: &str) -> bool {
        let forced = self.using_chatgpt_backend()
            && Self::needs_chatgpt_http_transport(&request.model);
        if forced || self.transport_health.requires_http(session_id) {
            return false;
        }
        self.transport_health.mark_interrupted(session_id);
        true
    }
}
