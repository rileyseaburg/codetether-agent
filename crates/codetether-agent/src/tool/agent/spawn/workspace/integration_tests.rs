//! Finished child work lands in the parent checkout as uncommitted edits only.

use super::super::allocate;
use super::support::{commit_data, fixture, git, head};
use crate::tool::ToolResult;

#[tokio::test]
async fn completed_child_is_delivered_as_uncommitted_parent_edits() {
    let repo = fixture();
    let original = head(repo.path());
    let child = allocate(repo.path()).await.unwrap();
    let commit = commit_data(&child.worktree, "reviewed child change\n");
    let guidance = child.guidance();
    assert!(guidance.contains("Parent uncommitted changes are NOT copied"));
    assert!(guidance.contains("Nothing is auto-merged"));
    let delivered = child.deliver(ToolResult::success("done")).await;
    assert!(delivered.success, "{}", delivered.output);
    assert!(delivered.output.contains("uncommitted changes"));
    assert_eq!(head(repo.path()), original, "parent history untouched");
    assert!(git(repo.path(), &["diff", "--cached", "--name-only"]).is_empty());
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "reviewed child change\n"
    );
    assert!(child.worktree.is_dir());
    assert_eq!(head(&child.worktree), commit);
}

#[tokio::test]
async fn failed_child_is_not_delivered() {
    let repo = fixture();
    let child = allocate(repo.path()).await.unwrap();
    commit_data(&child.worktree, "unverified change\n");
    let failure = child.attach(child.deliver(ToolResult::error("task failed")).await);
    assert!(!failure.success);
    assert!(failure.output.contains("deliver_uncommitted_to_parent"));
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "primary\n"
    );
}
