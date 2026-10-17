//! Moltbook client save key to vault.
use super::*;
impl MoltbookClient {
    // ------- credential management -----------------------------------------

    /// Store the API key in Vault so it persists across sessions.
    ///
    /// # Arguments
    /// * `api_key` - Credential to persist through the shared secrets manager.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// MoltbookClient::save_key_to_vault("example").await?;
    /// # Ok(()) }
    /// ```
    pub async fn save_key_to_vault(api_key: &str) -> Result<()> {
        let secrets = codetether_secrets::ProviderSecrets {
            api_key: Some(api_key.to_string()),
            base_url: Some(API_BASE.to_string()),
            organization: None,
            headers: None,
            extra: Default::default(),
        };
        codetether_secrets::set_provider_secrets(VAULT_PROVIDER_ID, &secrets)
            .await
            .context("Failed to store Moltbook API key in Vault")?;
        tracing::info!("Moltbook API key saved to Vault at codetether/moltbook");
        Ok(())
    }
}
