//! Budget prepared summaries before optional raw history, without duplicating ranges.
use super::super::{incremental_insert::summary_message, incremental_types::SummaryGap};
use crate::session::helper::token::estimate_tokens_for_messages;
use crate::session::index::SummaryIndex;

/// Pack stable prepared ranges first, then relevance-ranked raw messages.
pub(super) fn select(
    index: &SummaryIndex,
    keep: &mut [bool],
    per_message: &[usize],
    order: Vec<usize>,
    mut budget: usize,
) -> Vec<SummaryGap> {
    let mut covered = vec![false; keep.len()];
    let mut accepted = Vec::new();
    let dropped = super::collect_dropped_ranges(keep);
    for gap in super::prepared_gaps::select(index, &dropped) {
        let range = gap.range.start..gap.range.end;
        let raw_cost: usize = per_message[range.clone()].iter().sum();
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

#[cfg(test)]
#[path = "incremental_pack_tests.rs"]
mod tests;
