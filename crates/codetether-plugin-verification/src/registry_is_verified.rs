//! registry is verified for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Check if a plugin is registered and verified.
    pub async fn is_verified(&self, plugin_id: &str) -> bool {
        self.plugins.read().await.contains_key(plugin_id)
    }
}
