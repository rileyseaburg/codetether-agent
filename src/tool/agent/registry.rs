//! Provider registry access for agent tool actions.
//!
//! This module owns the cached provider registry used by agent actions.
//! It isolates provider bootstrap from messaging and spawn workflows.
//!
//! # Examples
//!
//! ```ignore
//! let registry = registry::get_registry().await?;
//! ```

use crate::provider::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

/// Returns a cached provider registry for agent-tool operations.
///
/// # Examples
///
/// ```ignore
/// let registry = get_registry().await?;
/// ```
pub(super) async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY
        .get_or_try_init(|| async { Ok(Arc::new(ProviderRegistry::from_vault().await?)) })
        .await
        .cloned()
}
