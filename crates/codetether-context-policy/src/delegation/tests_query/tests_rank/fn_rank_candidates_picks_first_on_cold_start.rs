//! rank candidates picks first on cold start policy.
use super::*;

#[test]
pub(super) fn rank_candidates_picks_first_on_cold_start() {
    let state = DelegationState::with_config(DelegationConfig::default());
    assert_eq!(
        state.rank_candidates(&["a", "b", "c"], "swarm_dispatch", bucket()),
        Some("a")
    );
}
