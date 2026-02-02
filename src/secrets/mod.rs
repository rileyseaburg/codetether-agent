//! Secrets management via HashiCorp Vault
//!
//! All API keys and secrets are loaded exclusively from HashiCorp Vault.
//! Environment variables are NOT used for secrets.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

/// Path in Vault where provider secrets are stored
const DEFAULT_SECRETS_PATH: &str = "secret/data/codetether/providers";

/// Vault-based secrets manager
#[derive(Clone)]
pub struct SecretsManager {
    client: Option<Arc<VaultClient>>,
    /// Cache of loaded API keys (provider_id -> api_key)
    pub cache: Arc<RwLock<HashMap<String, String>>>,
    mount: String,
    path: String,
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self {
            client: None,
            cache: Arc::new(RwLock::new(HashMap::new())),
            mount: "secret".to_string(),
            path: "codetether/providers".to_string(),
        }
    }
}

impl SecretsManager {
    /// Create a new secrets manager with Vault configuration
    pub async fn new(config: &VaultConfig) -> Result<Self> {
        let settings = VaultClientSettingsBuilder::default()
            .address(&config.address)
            .token(&config.token)
            .build()
            .context("Failed to build Vault client settings")?;

        let client = VaultClient::new(settings).context("Failed to create Vault client")?;

        Ok(Self {
            client: Some(Arc::new(client)),
            cache: Arc::new(RwLock::new(HashMap::new())),
            mount: config.mount.clone().unwrap_or_else(|| "secret".to_string()),
            path: config
                .path
                .clone()
                .unwrap_or_else(|| "codetether/providers".to_string()),
        })
    }

    /// Try to create from environment (for initial bootstrap only)
    pub async fn from_env() -> Result<Self> {
        let address = std::env::var("VAULT_ADDR").context("VAULT_ADDR not set")?;
        let token = std::env::var("VAULT_TOKEN").context("VAULT_TOKEN not set")?;
        let mount = std::env::var("VAULT_MOUNT").ok();
        let path = std::env::var("VAULT_SECRETS_PATH").ok();

        let config = VaultConfig {
            address,
            token,
            mount,
            path,
        };

        Self::new(&config).await
    }

    /// Check if Vault is configured and connected
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    /// Get an API key for a provider from Vault
    pub async fn get_api_key(&self, provider_id: &str) -> Result<Option<String>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(key) = cache.get(provider_id) {
                return Ok(Some(key.clone()));
            }
        }

        // Fetch from Vault
        let client = match &self.client {
            Some(c) => c,
            None => return Ok(None),
        };

        let secret_path = format!("{}/{}", self.path, provider_id);

        match kv2::read::<ProviderSecrets>(client.as_ref(), &self.mount, &secret_path).await {
            Ok(secret) => {
                // Cache the result
                if let Some(ref api_key) = secret.api_key {
                    let mut cache = self.cache.write().await;
                    cache.insert(provider_id.to_string(), api_key.clone());
                }
                Ok(secret.api_key)
            }
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(None),
            Err(e) => {
                tracing::warn!("Failed to fetch secret for {}: {}", provider_id, e);
                Ok(None)
            }
        }
    }

    /// Get all secrets for a provider
    pub async fn get_provider_secrets(&self, provider_id: &str) -> Result<Option<ProviderSecrets>> {
        let client = match &self.client {
            Some(c) => c,
            None => return Ok(None),
        };

        let secret_path = format!("{}/{}", self.path, provider_id);

        match kv2::read::<ProviderSecrets>(client.as_ref(), &self.mount, &secret_path).await {
            Ok(secret) => Ok(Some(secret)),
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(None),
            Err(e) => {
                tracing::warn!("Failed to fetch secrets for {}: {}", provider_id, e);
                Ok(None)
            }
        }
    }

    /// Check if a provider has an API key in Vault
    pub async fn has_api_key(&self, provider_id: &str) -> bool {
        match self.get_api_key(provider_id).await {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    /// List all providers that have secrets configured
    pub async fn list_configured_providers(&self) -> Result<Vec<String>> {
        let client = match &self.client {
            Some(c) => c,
            None => return Ok(Vec::new()),
        };

        match kv2::list(client.as_ref(), &self.mount, &self.path).await {
            Ok(keys) => Ok(keys),
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(Vec::new()),
            Err(e) => {
                tracing::warn!("Failed to list providers: {}", e);
                Ok(Vec::new())
            }
        }
    }

    /// Clear the cache (useful when secrets are rotated)
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

/// Vault configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VaultConfig {
    /// Vault server address (e.g., "https://vault.example.com:8200")
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

/// Provider secrets stored in Vault
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// Global secrets manager instance
static SECRETS_MANAGER: tokio::sync::OnceCell<SecretsManager> = tokio::sync::OnceCell::const_new();

/// Initialize the global secrets manager
pub async fn init_secrets_manager(config: &VaultConfig) -> Result<()> {
    let manager = SecretsManager::new(config).await?;
    SECRETS_MANAGER
        .set(manager)
        .map_err(|_| anyhow::anyhow!("Secrets manager already initialized"))?;
    Ok(())
}

/// Initialize the global secrets manager from an existing manager instance
pub fn init_from_manager(manager: SecretsManager) -> Result<()> {
    SECRETS_MANAGER
        .set(manager)
        .map_err(|_| anyhow::anyhow!("Secrets manager already initialized"))?;
    Ok(())
}

/// Get the global secrets manager
pub fn secrets_manager() -> Option<&'static SecretsManager> {
    SECRETS_MANAGER.get()
}

/// Get API key for a provider (convenience function)
pub async fn get_api_key(provider_id: &str) -> Option<String> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.get_api_key(provider_id).await.ok().flatten(),
        None => None,
    }
}

/// Check if a provider has an API key (convenience function)
pub async fn has_api_key(provider_id: &str) -> bool {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.has_api_key(provider_id).await,
        None => false,
    }
}

/// Get full provider secrets (convenience function)
pub async fn get_provider_secrets(provider_id: &str) -> Option<ProviderSecrets> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.get_provider_secrets(provider_id).await.ok().flatten(),
        None => None,
    }
}
