impl VaultCredentialStore {
    /// Read the current Codex OAuth credentials directly from Vault.
    async fn load(&self) -> Result<Option<OAuthCredentials>> {
        let manager = crate::secrets::secrets_manager().context("Vault is not configured")?;
        let secrets = manager.get_provider_secrets(&self.provider_id).await?;
        Ok(secrets
            .as_ref()
            .and_then(crate::provider::init_dispatch_impl::codex::credentials))
    }
}
