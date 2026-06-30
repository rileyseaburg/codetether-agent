//! Local HuggingFace model loading for automatic embedder selection.

use super::resources::SystemCapability;
use super::{BertEmbedder, catalog, hf_download};
use crate::vectordb::TextEmbedder;
use std::sync::Arc;

/// Attempt to download and load the best-fitting local model.
pub(super) async fn try_local(caps: &SystemCapability) -> Option<Arc<dyn TextEmbedder>> {
    let spec = catalog::best_fitting(caps.total_memory_bytes);
    let files = match hf_download::download(&spec).await {
        Ok(files) => files,
        Err(e) => {
            tracing::warn!(model = spec.repo, error = %e, "memory embedder: model download failed");
            return None;
        }
    };
    match tokio::task::spawn_blocking(move || BertEmbedder::load(&files)).await {
        Ok(Ok(embedder)) => {
            tracing::info!(
                model = spec.repo,
                dims = spec.dimensions,
                "memory embedder: local model loaded"
            );
            Some(Arc::new(embedder))
        }
        Ok(Err(e)) => {
            tracing::warn!(model = spec.repo, error = %e, "memory embedder: model load failed");
            None
        }
        Err(e) => {
            tracing::warn!(error = %e, "memory embedder: model load task panicked");
            None
        }
    }
}
