//! registry for verified plugin registration.
use super::*;

/// Plugin registry — tracks registered and verified plugins.
///
/// # Examples
/// ```rust,no_run
/// let _registry = codetether_plugin_verification::PluginRegistry::from_env();
/// ```
#[derive(Debug, Clone)]
pub struct PluginRegistry {
    pub(super) signing_key: SigningKey,
    /// Verified plugins: id -> manifest.
    pub(super) plugins: Arc<RwLock<HashMap<String, PluginManifest>>>,
}
