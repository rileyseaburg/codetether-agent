//! rank candidates prefers best scoring once warm policy.
use super::*;

#[test]
pub(super) fn rank_candidates_prefers_best_scoring_once_warm() {
    let mut state = DelegationState::with_config(DelegationConfig::default());
    let b = bucket();
    for _ in 0..5 {
        state.update("b", "swarm_dispatch", b, true);
    }
    for _ in 0..5 {
        state.update("a", "swarm_dispatch", b, false);
    }
    assert_eq!(
        state.rank_candidates(&["a", "b"], "swarm_dispatch", b),
        Some("b")
    );
}
