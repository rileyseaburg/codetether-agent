//! Verify the agent preserves worktree type identities through a crate re-export.

#[test]
fn agent_facade_reexports_the_worktree_crate_directly() {
    // A direct crate alias preserves every type's identity without compiling
    // the agent as a reverse dependency of its own worktree dependency.
    let source = include_str!("../../codetether-agent/src/lib.rs");
    assert!(
        source
            .lines()
            .any(|line| line.trim() == "pub use codetether_worktree as worktree;")
    );
}
