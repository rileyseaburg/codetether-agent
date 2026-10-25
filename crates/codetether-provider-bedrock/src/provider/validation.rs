//! Validate configured auth before provider requests are dispatched.
//!
//! [BedrockProvider] rejects empty bearer tokens or incomplete IAM key pairs
//! with the existing errors before invoking its client.
use super::{BedrockAuth, BedrockProvider};
use anyhow::Result;
impl BedrockProvider {
    /// Adopt an externally refreshed Vault key without minting or saving.
    /// A slow or unavailable Vault leaves existing credentials in place.
    ///
    /// # Errors
    /// Returns an error if the resulting credentials are empty or incomplete.
    pub(crate) async fn prepare_auth(&self) -> Result<()> {
        if tokio::time::timeout(
            std::time::Duration::from_secs(2),
            super::auth_recover::adopt_vault_key(self),
        )
        .await
        .is_err()
        {
            tracing::warn!(
                provider = "bedrock",
                "pre-request Vault key check timed out; retaining current credentials"
            );
        }
        self.validate_auth()
    }

    /// Validate that credentials/token are non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the bearer token is empty or AWS keys
    /// are incomplete.
    pub(crate) fn validate_auth(&self) -> Result<()> {
        match &self.auth {
            BedrockAuth::BearerToken(cell) => {
                if cell.read().is_empty() {
                    anyhow::bail!("Bedrock API key is empty");
                }
            }
            BedrockAuth::SigV4(creds) => {
                if creds.access_key_id.is_empty() || creds.secret_access_key.is_empty() {
                    anyhow::bail!("AWS credentials are incomplete");
                }
            }
        }
        Ok(())
    }
}
