//! registry list for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// List all registered plugins.
    pub async fn list(&self) -> Vec<PluginManifest> {
        self.plugins.read().await.values().cloned().collect()
    }
}
