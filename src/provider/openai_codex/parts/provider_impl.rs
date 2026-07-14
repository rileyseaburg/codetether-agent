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
        self.validate_model_for_backend(&request.model)?;
        let access_token = self.get_access_token().await?;
        if self.using_chatgpt_backend() {
            return self
                .complete_with_chatgpt_responses(request, access_token)
                .await;
        }
        self.complete_with_openai_responses(request, access_token)
            .await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.validate_model_for_backend(&request.model)?;
        let access_token = self.get_access_token().await?;
        if self.using_chatgpt_backend() {
            return self
                .complete_stream_with_chatgpt_responses(request, access_token)
                .await;
        }
        self.complete_stream_with_openai_responses(request, access_token)
            .await
    }
}
