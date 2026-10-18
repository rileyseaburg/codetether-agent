//! Provider completion and stream behavior for local CUDA inference.
use super::*;

#[async_trait]
impl<R: Runtime> Provider for LocalCudaProvider<R> {
    fn name(&self) -> &str {
        "local_cuda"
    }

    fn resolved_model_identity(&self, _model: &str) -> String {
        self.model_name.clone()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.model_catalog().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let (system_prompt, user_prompt) = Self::to_prompts(&request.messages);
        let runtime = self.runtime(&request).await?;
        let output = runtime
            .think_as_provider(self.name(), &system_prompt, &user_prompt)
            .await
            .with_context(|| {
                format!("local_cuda inference failed for model {}", self.model_name)
            })?;

        tracing::debug!(
            model = %self.model_name,
            prompt_tokens = output.prompt_tokens.unwrap_or(0),
            completion_tokens = output.completion_tokens.unwrap_or(0),
            finish_reason = ?output.finish_reason,
            "Local CUDA inference completed"
        );

        let text = Self::strip_inline_think_blocks(&output.text);
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text }],
            },
            usage: Usage {
                prompt_tokens: output.prompt_tokens.unwrap_or(0) as usize,
                completion_tokens: output.completion_tokens.unwrap_or(0) as usize,
                total_tokens: output.total_tokens.unwrap_or(0) as usize,
                cache_read_tokens: output.cache_read_tokens.map(|v| v as usize),
                cache_write_tokens: output.cache_write_tokens.map(|v| v as usize),
            },
            finish_reason: Self::map_finish_reason(output.finish_reason.as_deref()),
        })
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        Err(anyhow!(
            "Streaming inference not yet implemented for local_cuda provider"
        ))
    }
}
