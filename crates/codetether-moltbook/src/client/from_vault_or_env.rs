//! Moltbook client from vault or env.
use super::*;
impl MoltbookClient {
    /// Try to build a client by reading the key from Vault, then env var.
    ///
    /// # Arguments
    /// Uses the client credential and the documented endpoint.
    /// # Returns
    /// The operation result decoded or rendered as described above.
    /// # Errors
    /// Propagates credential, HTTP, unsuccessful-response, or decoding errors
    /// encountered by this operation.
    /// # Examples
    /// ```no_run
    /// use codetether_moltbook::MoltbookClient;
    /// # async fn example(client: &MoltbookClient) -> anyhow::Result<()> {
    /// let _ = MoltbookClient::from_vault_or_env().await?;
    /// # Ok(()) }
    /// ```
    pub async fn from_vault_or_env() -> Result<Self> {
        // 1. Try Vault
        if let Some(key) = codetether_secrets::get_api_key(VAULT_PROVIDER_ID).await
            && !key.is_empty()
        {
            tracing::info!("Moltbook API key loaded from Vault");
            return Ok(Self::new(key));
        }

        // 2. Try env var (local dev convenience)
        if let Ok(key) = std::env::var("MOLTBOOK_API_KEY")
            && !key.is_empty()
        {
            tracing::warn!(
                "Moltbook API key loaded from MOLTBOOK_API_KEY env var — \
                     consider storing it in Vault instead"
            );
            return Ok(Self::new(key));
        }

        anyhow::bail!(
            "No Moltbook API key found. Register first with `codetether moltbook register`"
        )
    }
}
