//! Budget prepared summaries before optional raw history, without duplicating ranges.
use crate::projection::{incremental_insert::summary_message, incremental_types::SummaryGap};
use crate::summary_index::SummaryIndex;
use codetether_provider_core::Message;

/// Pack stable prepared ranges first, then relevance-ranked raw messages.
pub fn select(
    index: &SummaryIndex,
    keep: &mut [bool],
    per_message: &[usize],
    order: Vec<usize>,
    mut budget: usize,
    estimate_tokens_for_messages: impl Fn(&[Message]) -> usize,
) -> Vec<SummaryGap> {
    let mut covered = vec![false; keep.len()];
    let mut accepted = Vec::new();
    let dropped = super::ranges::collect_dropped_ranges(keep);
    for gap in super::prepared_gaps::select(index, &dropped) {
        if gap.range.end > keep.len() {
            continue;
        }
        let range = gap.range.start..gap.range.end;
        if covered[range.clone()].iter().any(|&value| value) {
            continue;
        }
        let raw_cost: usize = range
            .clone()
            .filter(|&i| !keep[i])
            .map(|i| per_message[i])
            .sum();
        let cost = estimate_tokens_for_messages(&[summary_message(&gap)]);
        if cost <= budget && cost < raw_cost {
            budget -= cost;
            covered[range].fill(true);
            accepted.push(gap);
        }
    }
    for idx in order {
        let cost = per_message[idx];
        if !keep[idx] && !covered[idx] && cost <= budget {
            keep[idx] = true;
            budget -= cost;
        }
    }
    accepted
}
