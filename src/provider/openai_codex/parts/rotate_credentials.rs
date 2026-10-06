impl OpenAiCodexProvider {
    /// Spend the current refresh token and persist the rotated credentials.
    async fn rotate_credentials(
        &self,
        state: &mut OAuthCredentialState,
    ) -> Result<OAuthCredentials> {
        let previous_refresh_token = state.credentials.refresh_token.clone();
        let refreshed = self
            .request_refreshed_credentials(&previous_refresh_token)
            .await?;
        state.credentials = merge_refreshed_credentials(&state.credentials, refreshed);
        state.pending_refresh_token = self
            .credential_store
            .as_ref()
            .map(|_| previous_refresh_token);
        self.persist_pending_credentials(state).await?;
        Ok(state.credentials.clone())
    }
}

fn merge_refreshed_credentials(
    previous: &OAuthCredentials,
    mut refreshed: OAuthCredentials,
) -> OAuthCredentials {
    refreshed.id_token = refreshed.id_token.or_else(|| previous.id_token.clone());
    refreshed.chatgpt_account_id = refreshed
        .chatgpt_account_id
        .or_else(|| previous.chatgpt_account_id.clone());
    refreshed
}
