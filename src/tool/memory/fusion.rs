//! Reciprocal-rank fusion (RRF) for combining lexical and semantic rankings.
//!
//! RRF avoids mixing raw scores of incompatible scales: each ranking
//! contributes `1 / (k + rank)`, and the fused score is their sum. See
//! Cormack et al., "Reciprocal Rank Fusion outperforms Condorcet" (2009).

/// RRF damping constant; 60 is the value from the original paper.
pub const RRF_K: f32 = 60.0;

/// Convert a slice of `(id, score)` into id→rank (0-based, descending score).
pub fn rank_positions(scored: &mut [(String, f32)]) -> Vec<(String, usize)> {
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored
        .iter()
        .enumerate()
        .map(|(rank, (id, _))| (id.clone(), rank))
        .collect()
}

/// RRF contribution for a single rank position.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::memory::fusion::rrf_term;
///
/// // Better (lower) ranks contribute more.
/// assert!(rrf_term(0) > rrf_term(5));
/// ```
pub fn rrf_term(rank: usize) -> f32 {
    1.0 / (RRF_K + rank as f32)
}
