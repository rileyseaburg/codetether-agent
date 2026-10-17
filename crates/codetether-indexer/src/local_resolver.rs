//! Reject remote model selection in the standalone local-only entry point.

use crate::{EmbeddingProvider, EmbeddingResolver};
use std::sync::Arc;

pub(crate) struct LocalResolver;

#[async_trait::async_trait]
impl EmbeddingResolver for LocalResolver {
    async fn resolve(
        &self,
        selector: &str,
    ) -> anyhow::Result<(Arc<dyn EmbeddingProvider>, String)> {
        anyhow::bail!("remote embedding model '{selector}' requires an application resolver")
    }
}
