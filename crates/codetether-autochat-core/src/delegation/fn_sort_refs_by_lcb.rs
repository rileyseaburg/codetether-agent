//! sort refs by lcb policy.
use super::*;

/// Sort `model_refs` by descending LCB score.
///
/// Each entry is `"provider/model"` — we extract the provider name,
/// look up its score, and sort highest-first.  Entries with no
/// posterior keep their relative input order (stable sort).
pub fn sort_refs_by_lcb(state: &DelegationState, model_refs: &mut [String], bucket: Bucket) {
    if !state.enabled() || model_refs.len() <= 1 {
        return;
    }
    model_refs.sort_by(|a, b| {
        let sa = score_ref(state, a, bucket);
        let sb = score_ref(state, b, bucket);
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });
}
