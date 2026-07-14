//! Select prepared proactive-RLM summaries for dropped transcript ranges.

use crate::session::index::SummaryIndex;

use super::super::incremental_types::SummaryGap;

#[cfg(test)]
#[path = "prepared_gaps_tests.rs"]
mod tests;

pub(super) fn select(index: &SummaryIndex, dropped: &[(usize, usize)]) -> Vec<SummaryGap> {
    let mut gaps = Vec::new();
    for &(start, end) in dropped {
        let mut cursor = start;
        while cursor < end {
            let earliest = index
                .entries()
                .filter(|(range, _)| range.start >= cursor && range.end <= end)
                .map(|(range, _)| range.start)
                .min();
            let best = earliest.and_then(|start| {
                index
                    .entries()
                    .filter(|(range, _)| range.start == start && range.end <= end)
                    .max_by_key(|(range, _)| range.end.saturating_sub(range.start))
            });
            let Some((range, node)) = best else { break };
            gaps.push(SummaryGap {
                range: *range,
                content: node.content.clone(),
            });
            cursor = range.end;
        }
    }
    gaps.sort_by_key(|gap| (gap.range.start, gap.range.end));
    gaps
}
