//! Verify the agent facade preserves the extracted crate's type identities.

#[path = "../../../src/worktree/mod.rs"]
mod agent_worktree;

#[test]
fn manager_is_shared_with_the_agent_facade() {
    let manager: agent_worktree::WorktreeManager =
        codetether_worktree::WorktreeManager::for_repo("/srv/project");
    let manager: codetether_worktree::WorktreeManager = manager;

    assert_eq!(manager.repo_path(), std::path::Path::new("/srv/project"));
}

#[test]
fn lifecycle_and_cleanup_types_are_shared_with_the_agent_facade() {
    use std::any::TypeId;

    assert_eq!(
        TypeId::of::<agent_worktree::MergeResult>(),
        TypeId::of::<codetether_worktree::MergeResult>()
    );
    assert_eq!(
        TypeId::of::<agent_worktree::maintenance::WorktreeCleanupState>(),
        TypeId::of::<codetether_worktree::maintenance::WorktreeCleanupState>()
    );
}
