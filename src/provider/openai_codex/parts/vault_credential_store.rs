#[derive(Clone)]
struct VaultCredentialStore {
    provider_id: String,
}

impl VaultCredentialStore {
    fn new(provider_id: &str) -> Self {
        Self {
            provider_id: provider_id.to_string(),
        }
    }

    async fn persist(&self, previous_refresh: &str, credentials: &OAuthCredentials) -> Result<()> {
        let manager = crate::secrets::secrets_manager().context("Vault is not configured")?;
        let current = manager
            .get_provider_secrets(&self.provider_id)
            .await?
            .context("Codex credentials are missing from Vault")?;
        let stored_refresh = current
            .extra
            .get("refresh_token")
            .and_then(Value::as_str)
            .context("Codex refresh token is missing from Vault")?;
        if stored_refresh != previous_refresh {
            anyhow::bail!("Codex refresh token changed concurrently in Vault");
        }
        let updated = updated_vault_secret(current, credentials);
        manager
            .set_provider_secrets(&self.provider_id, &updated)
            .await?;
        tracing::info!(provider = %self.provider_id, "Persisted refreshed Codex credentials");
        Ok(())
    }

}
