//! Automatic embedding-backend selection.
//!
//! No user choice: probe the host, and if it can run a local transformer,
//! download the best-fitting recent HuggingFace embedding model and use it.
//! Otherwise fall back to a cloud embedding provider (OpenAI). If neither is
//! available, callers keep the dependency-free local hashing engine.

pub mod bert_embedder;
pub mod bert_forward;
pub mod catalog;
pub mod hf_download;
pub mod provider_pick;
pub mod resources;

#[cfg(test)]
mod tests;

use crate::provider::ProviderRegistry;
use crate::vectordb::TextEmbedder;
use bert_embedder::BertEmbedder;
use resources::SystemCapability;
use std::sync::Arc;

/// Select and construct the best available embedding backend for this host.
///
/// Returns `None` when neither a local model nor a cloud provider is usable,
/// signalling the caller to keep the built-in hashing engine.
pub async fn auto_embedder(registry: &ProviderRegistry) -> Option<Arc<dyn TextEmbedder>> {
    let caps = SystemCapability::detect();
    if caps.supports_local_embedding()
        && let Some(local) = try_local(&caps).await
    {
        return Some(local);
    }
    provider_pick::cloud_embedder(registry)
}

/// Attempt to download and load a local HuggingFace model.
async fn try_local(caps: &SystemCapability) -> Option<Arc<dyn TextEmbedder>> {
    let spec = catalog::best_fitting(caps.total_memory_bytes);
    let files = hf_download::download(&spec).await.ok()?;
    let embedder = tokio::task::spawn_blocking(move || BertEmbedder::load(&files))
        .await
        .ok()?
        .ok()?;
    Some(Arc::new(embedder))
}
