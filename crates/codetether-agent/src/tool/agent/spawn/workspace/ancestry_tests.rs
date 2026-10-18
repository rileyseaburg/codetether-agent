//! Nested children inherit their immediate parent's commit and relative cwd.

use super::super::allocate;
use super::support::{commit_data, fixture, head};

#[tokio::test]
async fn grandchild_uses_parent_head_and_primary_storage() {
    let repo = fixture();
    let root = repo.path().canonicalize().unwrap();
    let parent = allocate(&root.join("project")).await.unwrap();
    assert_eq!(parent.workspace, parent.worktree.join("project"));
    let parent_head = commit_data(&parent.worktree, "parent commit\n");
    assert_ne!(parent_head, head(&root));
    let child = allocate(&parent.workspace).await.unwrap();
    assert_eq!(child.base_commit, parent_head);
    assert_eq!(head(&child.worktree), parent_head);
    assert_eq!(child.parent_workspace, parent.workspace);
    assert_eq!(child.workspace, child.worktree.join("project"));
    assert_eq!(
        child.worktree.parent(),
        Some(root.join(".codetether-worktrees").as_path())
    );
    assert!(!child.worktree.starts_with(&parent.worktree));
    assert_eq!(
        std::fs::read_to_string(child.workspace.join("data.txt")).unwrap(),
        "parent commit\n"
    );
    assert_eq!(
        std::fs::read_to_string(root.join("project/data.txt")).unwrap(),
        "primary\n"
    );
    assert_eq!(head(&parent.worktree), parent_head);
}
