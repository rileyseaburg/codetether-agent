//! Explicit Vault connection and KV path settings.

/// Vault configuration
///
/// # Examples
/// ```
/// use codetether_secrets::{VaultConfig};
/// let config = VaultConfig::default();
/// assert_eq!(config.mount.as_deref(), Some("secret"));
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VaultConfig {
    /// Vault server address (e.g., `https://vault.example.com:8200`)
    pub address: String,

    /// Vault token for authentication
    pub token: String,

    /// KV secrets engine mount path (default: "secret")
    #[serde(default)]
    pub mount: Option<String>,

    /// Path prefix for provider secrets (default: "codetether/providers")
    #[serde(default)]
    pub path: Option<String>,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            address: String::new(),
            token: String::new(),
            mount: Some("secret".to_string()),
            path: Some("codetether/providers".to_string()),
        }
    }
}
