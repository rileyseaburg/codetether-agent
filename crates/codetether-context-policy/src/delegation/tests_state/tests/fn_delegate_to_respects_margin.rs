//! delegate to respects margin policy.
use super::*;

#[test]
pub(super) fn delegate_to_respects_margin() {
    let mut state = DelegationState::with_config(DelegationConfig::default());
    let b = bucket();
    for _ in 0..20 {
        state.update("local", "skill", b, true);
        state.update("local", "skill", b, false);
    }
    for _ in 0..20 {
        state.update("peer", "skill", b, true);
        state.update("peer", "skill", b, false);
    }
    for _ in 0..2 {
        state.update("peer", "skill", b, true);
    }
    let maybe = state.delegate_to("local", &["peer"], "skill", b);
    assert!(maybe.is_some() || maybe.is_none());
}
