impl OpenAiCodexProvider {
    async fn complete_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
        session_id: &str,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_chatgpt_responses(request, access_token, session_id)
            .await?;
        Self::collect_stream_completion(stream).await
    }
}
