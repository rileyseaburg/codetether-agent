//! registry get for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Get a plugin manifest.
    pub async fn get(&self, plugin_id: &str) -> Option<PluginManifest> {
        self.plugins.read().await.get(plugin_id).cloned()
    }
}
