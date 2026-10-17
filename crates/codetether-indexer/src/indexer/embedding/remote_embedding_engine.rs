//! remote embedding engine for codebase indexing.

use super::*;

#[derive(Clone)]
pub(in crate::indexer) struct RemoteEmbeddingEngine {
    pub(in crate::indexer) provider: Arc<dyn crate::EmbeddingProvider>,
    pub(in crate::indexer) provider_name: String,
    pub(in crate::indexer) model: String,
    pub(in crate::indexer) max_retries: u32,
    pub(in crate::indexer) retry_initial: Duration,
    pub(in crate::indexer) retry_max: Duration,
}

impl RemoteEmbeddingEngine {}
mod batch;
