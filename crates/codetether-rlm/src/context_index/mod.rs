//! Persistent, symbol-aware context indexing for RLM evidence.
//!
//! The index turns raw tool output into typed evidence records so the
//! engine retrieves relevant spans instead of compressing whole blobs.

mod cache;
mod extract;
mod fallback;
mod plan;
mod record;
mod retrieval;
mod score;
mod symbol;
#[cfg(test)]
mod tests;
mod text;
mod text_records;
mod types;

pub use types::{ContextIndex, EvidenceKind, EvidenceRecord, PlanIntent, RetrievalPlan};

/// Load a cached index for identical content or build and persist one.
pub fn load_or_build(source: &str, content: &str) -> ContextIndex {
    cache::load(source, content).unwrap_or_else(|| {
        let index = extract::build(source, content);
        cache::store(&index);
        index
    })
}

/// Retrieve the highest-value evidence records for `query`.
pub fn retrieve(index: &ContextIndex, query: &str, budget_tokens: usize) -> Vec<EvidenceRecord> {
    retrieval::retrieve(index, query, budget_tokens)
}
