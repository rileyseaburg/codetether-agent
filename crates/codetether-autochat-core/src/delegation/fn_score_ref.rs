//! score ref policy.
use super::*;

pub(super) fn score_ref(state: &DelegationState, model_ref: &str, bucket: Bucket) -> f64 {
    let (prov, _) = codetether_provider_core::routing::parse_model_string(model_ref);
    prov.and_then(|p| state.score(p, AUTOCHAT_MODEL_CALL, bucket))
        .unwrap_or(0.0)
}
