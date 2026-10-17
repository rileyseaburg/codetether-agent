//! Shared Vault client, cached keys, and authentication maintenance state.

use crate::{KubernetesAuthConfig, renewal};
use parking_lot::RwLock as ClientRwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use vaultrs::client::VaultClient;

/// Vault-based secrets manager
///
/// # Examples
/// ```
/// use codetether_secrets::{SecretsManager};
/// let manager = SecretsManager::default();
/// assert!(!manager.is_connected());
/// ```
#[derive(Clone)]
pub struct SecretsManager {
    pub(crate) client: Arc<ClientRwLock<Option<Arc<VaultClient>>>>,
    /// Cache of loaded API keys (provider_id -> api_key)
    pub cache: Arc<RwLock<HashMap<String, String>>>,
    pub(crate) mount: String,
    pub(crate) path: String,
    pub(crate) k8s_auth: Option<Arc<KubernetesAuthConfig>>,
    pub(crate) renewal: Arc<renewal::Monitor>,
}
