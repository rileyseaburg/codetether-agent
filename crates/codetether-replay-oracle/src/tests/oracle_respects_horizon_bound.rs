use super::fixtures::*;
use crate::replay_oracle;
use codetether_provider_core::Role;

#[test]
fn oracle_respects_horizon_bound() {
    let msgs = vec![
        text(Role::User, "edit src/lib.rs"),
        text(Role::Assistant, "noop"),
        text(Role::Assistant, "noop"),
        text(Role::User, "reopen src/lib.rs"),
    ];
    let short = replay_oracle(&msgs, 1);
    let long = replay_oracle(&msgs, 10);
    // With a tiny horizon, turn 0's future window barely extends.
    assert!(long.reference_count() >= short.reference_count());
}
