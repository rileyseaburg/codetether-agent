use super::fixtures::*;
use crate::replay_oracle;
use codetether_provider_core::Role;

#[test]
fn oracle_tracks_file_references_across_turns() {
    let msgs = vec![
        text(Role::User, "edit src/lib.rs"),
        text(Role::Assistant, "ok"),
        text(Role::User, "now open src/lib.rs again"),
    ];
    let report = replay_oracle(&msgs, 5);
    // Turn 2 re-mentions the file first seen at turn 0.
    assert!(report.demand[2].contains(&0));
}
