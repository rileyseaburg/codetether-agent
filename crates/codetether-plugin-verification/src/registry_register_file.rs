//! registry register file for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Register a plugin after verifying its signature and file content.
    pub async fn register_file(&self, manifest: PluginManifest, path: &Path) -> Result<()> {
        self.register_checked(manifest, Some(hash_file_async(path).await?))
            .await
    }
}
