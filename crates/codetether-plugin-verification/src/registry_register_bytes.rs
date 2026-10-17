//! registry register bytes for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Register a plugin after verifying its signature and byte content.
    pub async fn register_bytes(&self, manifest: PluginManifest, content: &[u8]) -> Result<()> {
        self.register_checked(manifest, Some(hash_bytes(content)))
            .await
    }
}
