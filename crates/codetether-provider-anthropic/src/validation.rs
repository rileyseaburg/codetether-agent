//! Validation helpers for Anthropic provider configuration.
//!
//! This module contains configuration checks that are shared by operations on
//! [`AnthropicProvider`]. Keeping validation here gives provider entry points a
//! single place to enforce required settings before listing models or sending
//! requests to an Anthropic-compatible backend.

use anyhow::Result;

use super::AnthropicProvider;

impl AnthropicProvider {
    /// Ensure an API key is present before making provider calls.
    ///
    /// The provider requires a non-empty API key for authenticated requests and
    /// for model-listing calls that should fail fast when credentials have not
    /// been configured. This method checks only presence; it does not contact
    /// the provider or verify that the key is accepted by the remote service.
    ///
    /// # Returns
    ///
    /// `Ok(())` when `self.api_key` is non-empty.
    ///
    /// # Errors
    ///
    /// Returns an error with the message `Anthropic API key is empty` when the
    /// configured credential is an empty string.
    ///
    /// # Side Effects
    ///
    /// This method performs no I/O and does not mutate the provider.
    pub(crate) fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Anthropic API key is empty");
        }
        Ok(())
    }
}
