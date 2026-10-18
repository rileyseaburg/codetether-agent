//! Construct one bounded evidence window with local search features.
use crate::RecallDocument;
use codetether_vectordb::LocalEmbeddingEngine;

pub(super) fn document(
    start: usize,
    messages: &[codetether_provider_core::Message],
    engine: &LocalEmbeddingEngine,
) -> RecallDocument {
    let excerpt = super::excerpt::render(messages, start);
    RecallDocument {
        start,
        end: start + messages.len(),
        tokens: super::tokens::document(&excerpt),
        embedding: engine.embed(&excerpt),
        excerpt,
    }
}
