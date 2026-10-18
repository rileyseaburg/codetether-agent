//! Child writes and commits leave parent files, index, and HEAD unchanged.

use super::super::allocate;
use super::support::{commit_data, fixture, git, head};

#[tokio::test]
async fn child_edits_and_commit_do_not_change_parent() {
    let repo = fixture();
    let original_head = head(repo.path());
    let child = allocate(repo.path()).await.unwrap();
    commit_data(&child.worktree, "child only\n");
    assert_eq!(head(repo.path()), original_head);
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "primary\n"
    );
    assert!(git(repo.path(), &["status", "--porcelain"]).is_empty());
    assert_ne!(child.workspace, child.parent_workspace);
    assert!(child.worktree.is_dir());
}

#[tokio::test]
async fn dirty_parent_index_worktree_and_untracked_files_are_not_copied() {
    let repo = fixture();
    let file = repo.path().join("project/data.txt");
    std::fs::write(&file, "staged parent\n").unwrap();
    git(repo.path(), &["add", "project/data.txt"]);
    std::fs::write(&file, "unstaged parent\n").unwrap();
    std::fs::write(repo.path().join("private.txt"), "untracked parent\n").unwrap();
    let status = git(repo.path(), &["status", "--porcelain"]);
    let index = git(repo.path(), &["diff", "--cached"]);
    let original_head = head(repo.path());
    let child = allocate(repo.path()).await.unwrap();
    assert_eq!(
        std::fs::read_to_string(child.worktree.join("project/data.txt")).unwrap(),
        "primary\n"
    );
    assert!(!child.worktree.join("private.txt").exists());
    assert!(git(&child.worktree, &["status", "--porcelain"]).is_empty());
    assert_eq!(child.base_commit, original_head);
    assert_eq!(head(repo.path()), original_head);
    assert_eq!(git(repo.path(), &["status", "--porcelain"]), status);
    assert_eq!(git(repo.path(), &["diff", "--cached"]), index);
    assert_eq!(std::fs::read_to_string(file).unwrap(), "unstaged parent\n");
}
