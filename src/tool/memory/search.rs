//! Ranking logic for [`MemoryStore::search`](super::MemoryStore::search).
//!
//! Hybrid retrieval: a lexical token-overlap filter selects candidates, then
//! reciprocal-rank fusion blends lexical and cached-embedding semantic
//! rankings. The query is embedded once and reused across all candidates.

use super::MemoryEntry;
use super::fuse::fuse;
use super::search_rank::{Candidate, make_candidate, matches_filters};
use crate::vectordb::{EmbeddingVector, LocalEmbeddingEngine};
use std::collections::HashMap;

/// Collect candidates passing the tag/scope filters and lexical query match.
pub(super) fn collect(
    entries: &mut HashMap<String, MemoryEntry>,
    query: Option<&str>,
    tags: Option<&[String]>,
    scope: Option<&str>,
) -> Vec<Candidate> {
    entries
        .values_mut()
        .filter(|entry| matches_filters(entry, tags, scope))
        .filter_map(|entry| make_candidate(entry, query))
        .collect()
}

/// Rank `candidates` against a precomputed `query_vec` and take the top-`limit`.
///
/// Use this when the query must be embedded with the same backend that
/// produced the entries' vectors (e.g. a provider-backed embedder).
pub(super) fn rank(
    candidates: Vec<Candidate>,
    query_vec: Option<&EmbeddingVector>,
    limit: usize,
) -> Vec<MemoryEntry> {
    let mut results = fuse(candidates, query_vec);
    results.truncate(limit);
    results
}

/// Filter `entries` by tags/scope, score them, and return the ranked top-`limit`.
///
/// Embeds the query with the local engine; for backend-consistent ranking use
/// [`collect`] + [`rank`] with a query vector from the same embedder.
pub fn run(
    entries: &mut HashMap<String, MemoryEntry>,
    query: Option<&str>,
    tags: Option<&[String]>,
    scope: Option<&str>,
    limit: usize,
) -> Vec<MemoryEntry> {
    let candidates = collect(entries, query, tags, scope);
    let query_vec = query
        .filter(|q| !q.trim().is_empty())
        .map(|q| LocalEmbeddingEngine::default().embed(q));
    rank(candidates, query_vec.as_ref(), limit)
}

#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;
