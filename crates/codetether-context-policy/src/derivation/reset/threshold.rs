//! Full-clone and latest-reset-marker threshold checks.
use super::*;
pub(super) fn check(
    mut state: ResetState,
    threshold_tokens: usize,
    estimate: impl Fn(&[Message]) -> usize,
) -> Result<DerivedContext, ResetState> {
    let est = estimate(&state.messages);
    if est <= threshold_tokens {
        state.provenance = vec!["reset_below_threshold".to_string()];
        return Ok(state.complete(false));
    }
    if let Some(reset_idx) =
        markers::latest_reset_marker_index(&state.messages).filter(|idx| *idx > 0)
    {
        state.messages = state.messages.split_off(reset_idx);
        state.base_index = reset_idx;
        state.dropped_ranges.push((0, reset_idx));
        state.provenance.push("reset_marker".to_string());
        let anchored_est = estimate(&state.messages);
        if anchored_est <= threshold_tokens {
            return Ok(state.complete(true));
        }
    }
    Err(state)
}
