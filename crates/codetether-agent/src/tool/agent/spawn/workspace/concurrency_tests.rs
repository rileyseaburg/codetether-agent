//! Concurrent allocations get different branches and primary-rooted paths.

use super::super::allocate;
use super::support::{fixture, head};

#[tokio::test]
async fn concurrent_children_have_unique_checkouts_and_branches() {
    let repo = fixture();
    let (left, right) = tokio::join!(allocate(repo.path()), allocate(repo.path()));
    let left = left.unwrap();
    let right = right.unwrap();
    let storage = repo
        .path()
        .canonicalize()
        .unwrap()
        .join(".codetether-worktrees");
    assert_ne!(left.worktree, right.worktree);
    assert_ne!(left.branch, right.branch);
    for child in [&left, &right] {
        assert_eq!(child.worktree.parent(), Some(storage.as_path()));
        assert_eq!(child.workspace, child.worktree);
        assert_eq!(head(&child.worktree), head(repo.path()));
    }
    std::fs::write(left.workspace.join("project/data.txt"), "left only\n").unwrap();
    assert_eq!(
        std::fs::read_to_string(right.workspace.join("project/data.txt")).unwrap(),
        "primary\n"
    );
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "primary\n"
    );
}
