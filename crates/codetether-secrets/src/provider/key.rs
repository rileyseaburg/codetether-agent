//! Credential presence and length inspection.

use crate::ProviderSecrets;

impl ProviderSecrets {
    /// Check if API key is present and valid (non-empty)
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{ProviderSecrets};
    /// let value = ProviderSecrets::default();
    /// assert!(!value.has_valid_api_key());
    /// ```
    pub fn has_valid_api_key(&self) -> bool {
        self.api_key
            .as_ref()
            .map(|k| !k.is_empty())
            .unwrap_or(false)
    }

    /// Get API key length without exposing the key
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{ProviderSecrets};
    /// let value = ProviderSecrets::default();
    /// assert_eq!(value.api_key_len(), None);
    /// ```
    pub fn api_key_len(&self) -> Option<usize> {
        self.api_key.as_ref().map(|k| k.len())
    }
}
