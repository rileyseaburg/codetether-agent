use std::path::Path;

#[test]
fn worktree_uses_a_direct_crate_reexport() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let monolith = root.join("src/worktree.rs");
    let split_mod = root.join("src/worktree/mod.rs");
    let source = std::fs::read_to_string(root.join("src/lib.rs")).unwrap();
    assert!(
        source
            .lines()
            .any(|line| line.trim() == "pub use codetether_worktree as worktree;")
    );
    assert!(!monolith.exists(), "obsolete worktree facade remains");
    assert!(!split_mod.exists(), "obsolete worktree facade remains");
}

#[test]
fn worktree_implementation_lives_in_workspace_crate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(root.join("../codetether-worktree/src/lib.rs").is_file());
    assert!(root.join("../codetether-worktree/src/manager.rs").is_file());
}
