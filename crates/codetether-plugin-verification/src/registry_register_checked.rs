//! registry register checked for verified plugin registration.
use super::*;
impl PluginRegistry {
    pub(super) async fn register_checked(
        &self,
        manifest: PluginManifest,
        actual_hash: Option<String>,
    ) -> Result<()> {
        if !self.signing_key.verify(&manifest) {
            return Err(anyhow!(
                "Plugin '{}' v{} has an invalid signature — refusing to register",
                manifest.id,
                manifest.version,
            ));
        }

        if let Some(actual_hash) = actual_hash {
            if actual_hash != manifest.content_hash {
                return Err(anyhow!(
                    "Plugin '{}' v{} content hash mismatch — refusing to register",
                    manifest.id,
                    manifest.version,
                ));
            }
            tracing::debug!(target: "codetether_agent::tool::sandbox",
                plugin_id = %manifest.id,
                manifest_hash = %manifest.content_hash,
                "Content hash verification completed"
            );
        } else {
            tracing::warn!(target: "codetether_agent::tool::sandbox",
                plugin_id = %manifest.id,
                "Plugin registered through explicit unsafe content verification bypass"
            );
        }

        tracing::info!(target: "codetether_agent::tool::sandbox",
            plugin_id = %manifest.id,
            version = %manifest.version,
            capabilities = ?manifest.capabilities,
            "Plugin registered and verified"
        );

        let mut plugins = self.plugins.write().await;
        plugins.insert(manifest.id.clone(), manifest);
        Ok(())
    }
}
