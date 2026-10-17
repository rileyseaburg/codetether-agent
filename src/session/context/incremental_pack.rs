//! Prepared-summary packing with the application token estimator.
#[cfg(test)]
use super::super::incremental_insert::summary_message;
use super::super::incremental_types::SummaryGap;
use crate::session::helper::token::estimate_tokens_for_messages;
use crate::session::index::SummaryIndex;
pub(super) fn select(
    index: &SummaryIndex,
    keep: &mut [bool],
    per_message: &[usize],
    order: Vec<usize>,
    budget: usize,
) -> Vec<SummaryGap> {
    codetether_context_policy::selection::pack::select(
        index,
        keep,
        per_message,
        order,
        budget,
        estimate_tokens_for_messages,
    )
}
#[cfg(test)]
#[path = "incremental_pack_tests.rs"]
mod tests;
