//! Choose a reset projection before any host summary retrieval.
use super::*;
/// Prepare a reset from an owned transcript and host request estimator.
///
/// # Arguments
/// * messages - Canonical transcript clone.
/// * origin_len - Canonical message count before preparation.
/// * threshold_tokens - Threshold that triggers prefix compression.
/// * estimate - Host request-token estimate including fixed overhead.
///
/// # Returns
/// Ready context, legacy fallback, or a prefix-summary plan.
///
/// # Examples
/// See the construction example on [ResetPreparation].
pub fn prepare(
    messages: Vec<Message>,
    origin_len: usize,
    threshold_tokens: usize,
    estimate: impl Fn(&[Message]) -> usize,
) -> ResetPreparation {
    let state = ResetState {
        messages,
        origin_len,
        dropped_ranges: Vec::new(),
        provenance: Vec::new(),
        base_index: 0,
    };
    match threshold::check(state, threshold_tokens, estimate) {
        Ok(context) => ResetPreparation::Ready(context),
        Err(state) => split::select(state),
    }
}
