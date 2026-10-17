//! registry verify content for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Verify a plugin's content hash against a file on disk.
    pub async fn verify_content(&self, plugin_id: &str, path: &Path) -> Result<bool> {
        let manifest = self
            .get(plugin_id)
            .await
            .ok_or_else(|| anyhow!("Plugin '{}' not registered", plugin_id))?;
        let file_hash = hash_file_async(path).await?;
        Ok(file_hash == manifest.content_hash)
    }
}
