//! embedding backend for codebase indexing.

use super::*;

pub(in crate::indexer) enum EmbeddingBackend {
    Local {
        engine: LocalEmbeddingEngine,
        model: String,
    },
    Remote(RemoteEmbeddingEngine),
}

impl EmbeddingBackend {
    pub(in crate::indexer) fn provider_name(&self) -> &str {
        match self {
            Self::Local { .. } => LOCAL_EMBEDDING_PROVIDER,
            Self::Remote(engine) => &engine.provider_name,
        }
    }

    pub(in crate::indexer) fn model_name(&self) -> &str {
        match self {
            Self::Local { model, .. } => model,
            Self::Remote(engine) => &engine.model,
        }
    }
}
