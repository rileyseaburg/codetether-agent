//! Select the active task tail and determine whether a summary is needed.
use super::*;
pub(super) fn select(mut state: ResetState) -> ResetPreparation {
    let Some(split_idx) = active_user_tail_start(&state.messages, 8) else {
        return if !state.provenance.is_empty() {
            ResetPreparation::Ready(state.complete(true))
        } else {
            ResetPreparation::Legacy
        };
    };
    let tail = state.messages.split_off(split_idx);
    let prefix = std::mem::take(&mut state.messages);
    if prefix.is_empty() {
        state.messages = tail;
        if state.provenance.is_empty() {
            state.provenance = vec!["reset_without_prefix".to_string()];
        }
        let compressed = state.base_index > 0;
        return ResetPreparation::Ready(state.complete(compressed));
    }
    ResetPreparation::Summarize(ResetSummaryPlan {
        prefix,
        tail,
        origin_len: state.origin_len,
        base_index: state.base_index,
        dropped_ranges: state.dropped_ranges,
        provenance: state.provenance,
    })
}
