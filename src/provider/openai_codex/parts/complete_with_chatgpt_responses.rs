impl OpenAiCodexProvider {
    async fn complete_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_chatgpt_responses(request, access_token)
            .await?;
        Self::collect_stream_completion(stream).await
    }
}
