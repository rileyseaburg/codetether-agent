//! Provider and executable-tool registry construction for one-shot children.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc};
/// Existing providers and tool registries used by ephemeral setup.
/// # Implementors
/// Applications delegate to their initialized providers and autonomous registries.
#[async_trait]
pub trait EphemeralResources: Send + Sync {
    /// Native executable tool registry.
    type Registry: Send + Sync;
    /// Resolve the selected provider/model using the current registry.
    /// Propagates native provider lookup errors.
    async fn resolve(model: &str) -> Result<(Arc<dyn Provider>, String)>;
    /// Build the existing read/write-capability-aware tool registry.
    fn registry(
        read_only: bool,
        verification: bool,
        workspace: &Path,
        provider: Arc<dyn Provider>,
        model: String,
    ) -> Arc<Self::Registry>;
}
