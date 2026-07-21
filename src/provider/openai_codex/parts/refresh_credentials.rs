impl OpenAiCodexProvider {
    async fn refresh_credentials(&self, rejected: Option<&str>) -> Result<OAuthCredentials> {
        let slot = self
            .stored_credentials
            .as_ref()
            .context("No OAuth credentials available. Run OAuth flow first.")?;
        let mut state = slot.write().await;
        self.persist_pending_credentials(&mut state).await?;
        let now = Self::oauth_expiry(0)?;
        if !refresh_required(&state.credentials, now, rejected) {
            return Ok(state.credentials.clone());
        }

        let previous_refresh_token = state.credentials.refresh_token.clone();
        let refreshed = self
            .request_refreshed_credentials(&previous_refresh_token)
            .await?;
        let refreshed = merge_refreshed_credentials(&state.credentials, refreshed);
        state.credentials = refreshed.clone();
        state.pending_refresh_token = self.credential_store.as_ref().map(|_| previous_refresh_token);
        self.persist_pending_credentials(&mut state).await?;
        Ok(refreshed)
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
