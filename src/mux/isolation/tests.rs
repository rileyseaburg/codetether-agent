use super::worktree::primary_checkout;
use std::path::Path;

#[path = "tests_workspace.rs"]
mod workspace;

#[test]
fn managed_checkout_resolves_to_primary_repository() {
    let repo = Path::new("/repo/.codetether-worktrees/mux-work-1-abcd");
    assert_eq!(primary_checkout(repo), Path::new("/repo"));
}

#[test]
fn primary_repository_is_unchanged() {
    assert_eq!(primary_checkout(Path::new("/repo")), Path::new("/repo"));
}

#[test]
fn no_worktree_flag_selects_shared_isolation() {
    assert!(super::Isolation::from_no_worktree(true).is_shared());
    assert!(!super::Isolation::from_no_worktree(false).is_shared());
    assert_eq!(super::Isolation::default(), super::Isolation::Worktree);
}
