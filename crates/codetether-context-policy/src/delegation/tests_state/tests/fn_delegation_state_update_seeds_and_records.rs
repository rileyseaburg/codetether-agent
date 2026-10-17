//! delegation state update seeds and records policy.
use super::*;

#[test]
pub(super) fn delegation_state_update_seeds_and_records() {
    let mut state = DelegationState::with_config(DelegationConfig::default());
    state.update("openai", "model_call", bucket(), true);
    let score = state
        .score("openai", "model_call", bucket())
        .expect("update must seed the posterior");
    assert!(score.is_finite());
}
