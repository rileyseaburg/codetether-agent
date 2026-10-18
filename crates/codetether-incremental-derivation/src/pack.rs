//! Prepared-summary packing with the application token estimator.
use codetether_context_policy::projection::incremental_types::SummaryGap;
use codetether_context_policy::summary_index::SummaryIndex;
use codetether_request_tokens::estimate_tokens_for_messages;
pub fn select(
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
