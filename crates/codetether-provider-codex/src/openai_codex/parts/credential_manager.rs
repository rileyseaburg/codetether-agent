impl OpenAiCodexProvider {
    fn credential_manager(&self) -> codetether_codex_auth::lifecycle::CredentialManager<'_> {
        codetether_codex_auth::lifecycle::CredentialManager::new(
            &self.client,
            &self.chatgpt_account_id,
            &self.stored_credentials,
            &self.credential_store,
        )
    }
}
