impl OpenAiCodexProvider {
    async fn complete_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
        session_id: &str,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_openai_responses(request, api_key, session_id)
            .await?;
        Self::collect_stream_completion(stream).await
    }
}
