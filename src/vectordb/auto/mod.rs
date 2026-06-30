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
pub mod local_pick;
pub mod provider_pick;
pub mod resources;

#[cfg(test)]
mod tests;

use crate::provider::ProviderRegistry;
use crate::vectordb::TextEmbedder;
use bert_embedder::BertEmbedder;
use local_pick::try_local;
use resources::SystemCapability;
use std::sync::Arc;

/// Select and construct the best available embedding backend for this host.
///
/// Returns `None` when neither a local model nor a cloud provider is usable,
/// signalling the caller to keep the built-in hashing engine.
pub async fn auto_embedder(registry: &ProviderRegistry) -> Option<Arc<dyn TextEmbedder>> {
    let caps = SystemCapability::detect();
    if caps.supports_local_embedding() {
        match try_local(&caps).await {
            Some(local) => return Some(local),
            None => tracing::warn!("memory embedder: local model unavailable, trying cloud"),
        }
    }
    match provider_pick::cloud_embedder(registry) {
        Some(cloud) => {
            tracing::info!("memory embedder: using cloud provider");
            Some(cloud)
        }
        None => {
            tracing::info!("memory embedder: using local hashing engine");
            None
        }
    }
}
