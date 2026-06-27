//! One-time automatic embedding-backend installation for the memory store.

use super::MemoryStore;
use crate::provider::ProviderRegistry;
use crate::vectordb::Embeddable;
use crate::vectordb::auto::auto_embedder;

impl MemoryStore {
    /// Install the best available embedding backend, chosen automatically.
    ///
    /// Probes system resources and either loads a local HuggingFace model or
    /// uses a cloud embedding provider. Existing entries are re-embedded with
    /// the chosen backend so queries and entries share one space. A failure to
    /// build any backend leaves the store on its hashing engine.
    pub async fn install_auto_embedder(mut self) -> Self {
        let registry = match ProviderRegistry::from_vault().await {
            Ok(registry) => registry,
            Err(_) => ProviderRegistry::new(),
        };
        let Some(embedder) = auto_embedder(&registry).await else {
            return self;
        };
        let texts: Vec<String> = self.entries.values().map(|e| e.embedding_text()).collect();
        match embedder.embed_batch(&texts).await {
            Ok(vectors) => {
                for (entry, vector) in self.entries.values_mut().zip(vectors) {
                    entry.embedding = Some(vector);
                }
                let _ = self.save().await;
            }
            Err(e) => tracing::warn!(error = %e, "re-embed of existing entries failed"),
        }
        self.with_embedder(embedder)
    }
}
