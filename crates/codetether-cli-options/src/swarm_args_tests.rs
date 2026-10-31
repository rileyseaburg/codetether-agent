//! Swarm sharing defaults and explicit isolation opt-in.
use super::SwarmArgs;
use clap::Parser;

#[test]
fn swarm_shares_by_default_and_accepts_explicit_isolation() {
    let shared = SwarmArgs::try_parse_from(["swarm", "Task"]).unwrap();
    assert!(!shared.isolate_worktrees);
    let isolated = SwarmArgs::try_parse_from(["swarm", "--isolate-worktrees", "Task"]).unwrap();
    assert!(isolated.isolate_worktrees);
}
