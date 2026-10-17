//! Serialized provider credentials and extension fields.

use std::collections::HashMap;

/// Provider secrets stored in Vault
///
/// # Examples
/// ```
/// use codetether_secrets::{ProviderSecrets};
/// let value = ProviderSecrets::default();
/// assert!(!value.has_valid_api_key());
/// assert_eq!(value.api_key_len(), None);
/// ```
#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProviderSecrets {
    /// API key for the provider
    #[serde(default)]
    pub api_key: Option<String>,

    /// Base URL override
    #[serde(default)]
    pub base_url: Option<String>,

    /// Organization ID (for OpenAI)
    #[serde(default)]
    pub organization: Option<String>,

    /// Additional headers as JSON
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,

    /// Any provider-specific extra fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
