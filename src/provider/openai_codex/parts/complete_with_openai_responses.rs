impl OpenAiCodexProvider {
    async fn complete_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_openai_responses(request, api_key)
            .await?;
        Self::collect_stream_completion(stream).await
    }
}
