//! rank candidates is none for empty input policy.
use super::*;

#[test]
pub(super) fn rank_candidates_is_none_for_empty_input() {
    let state = DelegationState::with_config(DelegationConfig::default());
    assert!(
        state
            .rank_candidates(&[], "swarm_dispatch", bucket())
            .is_none()
    );
}
