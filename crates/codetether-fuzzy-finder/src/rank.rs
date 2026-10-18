//! Rank candidates using the shared subsequence scorer.
use crate::FuzzyEntry;
use codetether_util::fuzzy::fuzzy_score;

/// Ranks `candidates` against `query`, writing the sorted matches into `out`.
/// # Arguments
/// * `query` - Case-insensitive subsequence to find.
/// * `candidates` - Original candidate catalog.
/// * `out` - Destination replaced with ranked matching candidates.
/// # Returns
/// Unit after sorting by descending score, then display label.
/// # Examples
/// ```
/// let mut out = Vec::new();
/// codetether_fuzzy_finder::rerank_into("main", &[], &mut out);
/// assert!(out.is_empty());
/// ```
pub fn rerank_into(query: &str, candidates: &[FuzzyEntry], out: &mut Vec<FuzzyEntry>) {
    if query.is_empty() {
        *out = candidates.to_vec();
        return;
    }
    let mut scored: Vec<(u32, &FuzzyEntry)> = candidates
        .iter()
        .filter_map(|c| fuzzy_score(query, &c.display).map(|s| (s, c)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.display.cmp(&b.1.display)));
    *out = scored.into_iter().map(|(_, c)| c.clone()).collect();
}
