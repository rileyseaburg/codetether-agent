//! Automatic embedding-backend selection.
//!
//! No user choice: probe the host, and if it can run a local transformer,
//! download the best-fitting recent HuggingFace embedding model and use it.
//! Otherwise fall back to a cloud embedding provider (OpenAI). If neither is
//! available, callers keep the dependency-free local hashing engine.

pub mod catalog;
pub mod hf_download;
pub mod provider_pick;
pub mod resources;

#[cfg(feature = "candle")]
pub mod bert_embedder;
#[cfg(feature = "candle")]
pub mod bert_forward;
#[cfg(feature = "candle")]
pub mod local_pick;

#[cfg(test)]
mod tests;

pub use registry::Registry;
mod registry;
use codetether_vectordb::TextEmbedder;
use resources::SystemCapability;
use std::sync::Arc;

#[cfg(feature = "candle")]
use bert_embedder::BertEmbedder;
#[cfg(feature = "candle")]
use local_pick::try_local;

/// Try a local transformer embedder; always `None` without the `candle` feature.
#[cfg(not(feature = "candle"))]
async fn try_local(_caps: &SystemCapability) -> Option<Arc<dyn TextEmbedder>> {
    None
}

/// Select and construct the best available embedding backend for this host.
///
/// Returns `None` when neither a local model nor a cloud provider is usable,
/// signalling the caller to keep the built-in hashing engine.
///
/// # Arguments
/// * `registry` — Existing provider lookup used only after local selection fails.
/// # Returns
/// A local or cloud embedder, or `None` to retain the caller's hashing engine.
/// # Examples
/// ```no_run
/// # async fn select(registry: &impl codetether_embedding_backends::auto::Registry) {
/// let embedder = codetether_embedding_backends::auto::auto_embedder(registry).await;
/// # let _ = embedder;
/// # }
/// ```
pub async fn auto_embedder(registry: &impl Registry) -> Option<Arc<dyn TextEmbedder>> {
    let caps = SystemCapability::detect();
    if caps.supports_local_embedding() {
        match try_local(&caps).await {
            Some(local) => return Some(local),
            None => {
                tracing::warn!(target: "codetether_agent::vectordb::auto", "memory embedder: local model unavailable, trying cloud")
            }
        }
    }
    match provider_pick::cloud_embedder(registry) {
        Some(cloud) => {
            tracing::info!(target: "codetether_agent::vectordb::auto", "memory embedder: using cloud provider");
            Some(cloud)
        }
        None => {
            tracing::info!(target: "codetether_agent::vectordb::auto", "memory embedder: using local hashing engine");
            None
        }
    }
}
