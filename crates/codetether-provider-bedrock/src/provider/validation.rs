//! Validate configured auth before provider requests are dispatched.
//!
//! [BedrockProvider] rejects empty bearer tokens or incomplete IAM key pairs
//! with the existing errors before invoking its client.
use super::{BedrockAuth, BedrockProvider};
use anyhow::Result;
impl BedrockProvider {
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
