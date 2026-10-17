//! shrink cold start pulls neighbour mass policy.
use super::*;

#[test]
pub(super) fn shrink_cold_start_pulls_neighbour_mass() {
    let mut state = DelegationState::with_config(DelegationConfig::default());
    let b1 = bucket();
    let b2 = Bucket {
        difficulty: Difficulty::Medium,
        ..b1
    };
    for _ in 0..10 {
        state.update("agent", "skill", b2, true);
    }
    state.shrink_cold_start("agent", "skill", b1, &[b2], 2.0);
    let post = state
        .beliefs
        .get(&DelegationState::key("agent", "skill", b1))
        .unwrap();
    assert!(post.alpha > post.beta);
}
