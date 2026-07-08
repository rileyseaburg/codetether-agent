//! Scope-widening search for memory entries saved before the stable-git-scope
//! refactor (commit 115d61e1). Pre-refactor entries used ad-hoc scopes like
//! `path:/home/<user>` and are invisible to stable-scope exact-match filters.
//!
//! Strategy: run the primary scoped search; if it returns nothing and a scope
//! was requested, retry without the scope filter so orphaned entries surface.

use std::collections::HashMap;

use crate::vectordb::EmbeddingVector;

use super::search_rank::Candidate;
use super::{MemoryEntry, search};

/// Search with stable scope; widen to unscoped on empty result.
pub(super) fn run(
    entries: &mut HashMap<String, MemoryEntry>,
    query: Option<&str>,
    tags: Option<&[String]>,
    scope: Option<&str>,
    query_vec: Option<&EmbeddingVector>,
    limit: usize,
) -> Vec<MemoryEntry> {
    let primary: Vec<Candidate> = search::collect(entries, query, tags, scope);
    let results = search::rank(primary, query_vec, limit);
    if !results.is_empty() || scope.is_none() {
        return results;
    }
    // Widen: drop scope so pre-migration entries are reachable.
    let wide: Vec<Candidate> = search::collect(entries, query, tags, None);
    search::rank(wide, query_vec, limit)
}
