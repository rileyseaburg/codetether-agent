use crate::worktree::WorktreeManager;

#[test]
fn detects_concurrent_gc_as_benign() {
    let output = "fatal: gc is already running on machine 'ubuntu-dev' pid 783660";
    assert!(WorktreeManager::is_benign_repair_failure(output));
}

#[test]
fn treats_real_failures_as_actionable() {
    let output = "fatal: unable to read tree 87d854d2";
    assert!(!WorktreeManager::is_benign_repair_failure(output));
}
