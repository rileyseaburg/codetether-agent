//! sort by lcb is noop when disabled policy.
use super::*;

#[test]
pub(super) fn sort_by_lcb_is_noop_when_disabled() {
    let state = DelegationState::with_config(DelegationConfig::default());
    let mut refs = vec!["b/model".into(), "a/model".into()];
    let b = bucket_for_turn("easy", false);
    sort_refs_by_lcb(&state, &mut refs, b);
    assert_eq!(refs[0], "b/model");
}
