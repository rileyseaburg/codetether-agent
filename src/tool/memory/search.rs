//! Ranking logic for [`MemoryStore::search`](super::MemoryStore::search).
//!
//! Hybrid retrieval: a lexical token-overlap filter selects candidates, then
//! reciprocal-rank fusion blends lexical and cached-embedding semantic
//! rankings. The query is embedded once and reused across all candidates.

use super::MemoryEntry;
use super::fuse::fuse;
use super::search_rank::{make_candidate, matches_filters};
use crate::vectordb::LocalEmbeddingEngine;
use std::collections::HashMap;

/// Filter `entries` by tags/scope, score them, and return the ranked top-`limit`.
pub fn run(
    entries: &mut HashMap<String, MemoryEntry>,
    query: Option<&str>,
    tags: Option<&[String]>,
    scope: Option<&str>,
    limit: usize,
) -> Vec<MemoryEntry> {
    let candidates: Vec<_> = entries
        .values_mut()
        .filter(|entry| matches_filters(entry, tags, scope))
        .filter_map(|entry| make_candidate(entry, query))
        .collect();

    let query_vec = query
        .filter(|q| !q.trim().is_empty())
        .map(|q| LocalEmbeddingEngine::default().embed(q));

    let mut results = fuse(candidates, query_vec.as_ref());
    results.truncate(limit);
    results
}

#[cfg(test)]
mod tests {
    use super::super::{MemoryEntry, MemoryStore};

    fn embedded(content: &str) -> MemoryEntry {
        let mut e = MemoryEntry::new(content, vec![]);
        e.ensure_embedding();
        e
    }

    #[test]
    fn ranks_semantically_closest_first() {
        let mut store = MemoryStore::default();
        // Both share token "runtime" so both pass the lexical filter; the
        // semantically closer tokio entry should win after fusion.
        store.add(embedded("java jvm runtime garbage collection"));
        store.add(embedded("rust async tokio runtime tasks"));

        let results = store.search(Some("tokio async runtime"), None, None, 2);
        assert_eq!(results.len(), 2);
        assert!(results[0].content.contains("tokio"));
    }
}
