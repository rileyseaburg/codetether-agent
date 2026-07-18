#[async_trait]
impl Provider for OpenAiCodexProvider {
    fn name(&self) -> &str {
        "openai-codex"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models = vec![
            // ChatGPT backend: gpt-5.5 plus Fast mode via service_tier=priority.
            // Authenticated Codex catalog: Sol/Terra support max+ultra; Luna supports max.
            // All three are available through the ChatGPT backend and Bedrock aliases.
            Self::model_info("gpt-5.5", "GPT-5.5", 272_000, 128_000, false),
            Self::model_info("gpt-5.5-fast", "GPT-5.5 Fast", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-sol", "GPT-5.6 Sol", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-terra", "GPT-5.6 Terra", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-luna", "GPT-5.6 Luna", 272_000, 128_000, false),
        ];

        if self.using_chatgpt_backend() {
            models.retain(|model| Self::chatgpt_supported_models().contains(&model.id.as_str()));
        } else {
            models.retain(|model| !Self::api_key_hidden_model(&model.id));
        }

        Ok(models)
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
}
