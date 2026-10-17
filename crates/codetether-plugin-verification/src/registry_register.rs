//! registry register for verified plugin registration.
use super::*;
impl PluginRegistry {
    /// Register a plugin manifest only when unsafe content bypass is enabled.
    pub async fn register(&self, manifest: PluginManifest) -> Result<()> {
        if !allow_unverified_content_registration() {
            return Err(anyhow!(
                "Plugin '{}' v{} requires content verification; use register_bytes or register_file",
                manifest.id,
                manifest.version,
            ));
        }
        self.register_checked(manifest, None).await
    }
}
