//! Persisted reset-threshold projection before model-relative resolution.
use super::DerivePolicy;

pub(super) fn resolve_reset_threshold(persisted: DerivePolicy, model: &str) -> usize {
    let persisted_tokens = match persisted {
        DerivePolicy::Reset { threshold_tokens } => Some(threshold_tokens),
        DerivePolicy::Legacy
        | DerivePolicy::Incremental { .. }
        | DerivePolicy::OracleReplay { .. } => None,
    };
    crate::derivation::reset_threshold::resolve_for_model(model, persisted_tokens)
}
