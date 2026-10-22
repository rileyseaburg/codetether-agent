//! Order prepared summary candidates that cover omitted history.
use crate::projection::incremental_types::SummaryGap;
use crate::summary_index::SummaryIndex;
#[cfg(test)]
#[path = "prepared_budget_tests.rs"]
mod budget_tests;
#[cfg(test)]
#[path = "prepared_overlap_tests.rs"]
mod overlap_tests;
#[cfg(test)]
#[path = "prepared_gaps_tests.rs"]
mod tests;

/// Order candidates covering omitted turns, including retained-tail overlaps.
/// The budget packer chooses affordable, nonoverlapping candidates; a large
/// unaffordable summary must not hide smaller alternatives covering its range.
/// # Arguments
/// `index` contains prepared summaries; `dropped` contains omitted source ranges.
/// # Returns
/// Candidates in source order, largest first for equal starting positions.
/// # Examples
/// ```
/// use codetether_context_policy::{selection::prepared_gaps, summary_index::SummaryIndex};
/// assert!(prepared_gaps::select(&SummaryIndex::new(), &[(0, 8)]).is_empty());
/// ```
pub fn select(index: &SummaryIndex, dropped: &[(usize, usize)]) -> Vec<SummaryGap> {
    let mut candidates: Vec<_> = index
        .entries()
        .filter(|(range, _)| {
            dropped
                .iter()
                .any(|&(start, end)| range.start < end && start < range.end)
        })
        .collect();
    candidates.sort_by_key(|(range, _)| (range.start, std::cmp::Reverse(range.end)));
    candidates
        .into_iter()
        .map(|(range, node)| SummaryGap {
            range: *range,
            content: node.content.clone(),
        })
        .collect()
}
