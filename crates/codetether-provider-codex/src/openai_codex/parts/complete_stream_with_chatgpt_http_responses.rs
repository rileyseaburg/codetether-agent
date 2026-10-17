impl OpenAiCodexProvider {
    async fn complete_stream_with_chatgpt_http_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
        account_id: String,
        session_id: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let mut response = self
            .send_chatgpt_http_response(&request, &access_token, &account_id, session_id)
            .await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            let access_token = self.force_refresh_access_token(&access_token).await?;
            let account_id = self
                .resolved_chatgpt_account_id(&access_token)
                .unwrap_or(account_id);
            response = self
                .send_chatgpt_http_response(&request, &access_token, &account_id, session_id)
                .await?;
        }
        Self::validate_http_response(response, &request.model).await
    }
}
