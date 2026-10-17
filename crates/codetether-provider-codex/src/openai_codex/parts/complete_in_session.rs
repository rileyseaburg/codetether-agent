impl OpenAiCodexProvider {
    async fn complete_in_session(
        &self,
        request: CompletionRequest,
        session_id: &str,
    ) -> Result<CompletionResponse> {
        self.validate_model_for_backend(&request.model)?;
        let access_token = self.get_access_token().await?;
        if self.using_chatgpt_backend() {
            return self
                .complete_with_chatgpt_responses(request, access_token, session_id)
                .await;
        }
        self.complete_with_openai_responses(request, access_token, session_id)
            .await
    }
}
