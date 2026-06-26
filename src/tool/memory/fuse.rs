//! Fuse lexical and semantic rankings into a final ordering.

use super::fusion::{rank_positions, rrf_term};
use super::search_rank::Candidate;
use super::semantic::score_against;
use crate::vectordb::EmbeddingVector;
use std::collections::HashMap;

/// Rank `candidates` via reciprocal-rank fusion of lexical + semantic signals.
///
/// `query_vec` is `None` for tag/scope-only listing, where semantic ranking is
/// skipped and lexical order (then importance) prevails.
pub(super) fn fuse(
    candidates: Vec<Candidate>,
    query_vec: Option<&EmbeddingVector>,
) -> Vec<super::MemoryEntry> {
    let mut lexical: Vec<(String, f32)> = candidates
        .iter()
        .map(|c| (c.entry.id.clone(), c.lexical as f32))
        .collect();
    let mut semantic: Vec<(String, f32)> = candidates
        .iter()
        .map(|c| {
            let s = query_vec.map(|q| score_against(&c.entry, q)).unwrap_or(0.0);
            (c.entry.id.clone(), s)
        })
        .collect();

    let mut fused: HashMap<String, f32> = HashMap::new();
    for (id, rank) in rank_positions(&mut lexical) {
        *fused.entry(id).or_insert(0.0) += rrf_term(rank);
    }
    if query_vec.is_some() {
        for (id, rank) in rank_positions(&mut semantic) {
            *fused.entry(id).or_insert(0.0) += rrf_term(rank);
        }
    }

    let mut ranked = candidates;
    ranked.sort_by(|a, b| {
        let sa = fused.get(&a.entry.id).copied().unwrap_or(0.0);
        let sb = fused.get(&b.entry.id).copied().unwrap_or(0.0);
        sb.partial_cmp(&sa)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.entry.importance.cmp(&a.entry.importance))
            .then_with(|| b.entry.access_count.cmp(&a.entry.access_count))
    });
    ranked.into_iter().map(|c| c.entry).collect()
}
