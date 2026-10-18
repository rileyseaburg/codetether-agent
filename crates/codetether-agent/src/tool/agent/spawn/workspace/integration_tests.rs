//! Integration happens only after parent review and an explicit cherry-pick.

use super::super::allocate;
use super::support::{commit_data, fixture, git, head};
use crate::tool::ToolResult;

#[tokio::test]
async fn parent_reviews_then_explicitly_cherry_picks_child_commit() {
    let repo = fixture();
    let original = head(repo.path());
    let child = allocate(repo.path()).await.unwrap();
    let commit = commit_data(&child.worktree, "reviewed child change\n");
    let failure = child.attach(ToolResult::error("task failed after committing"));
    assert!(!failure.success);
    assert!(failure.output.contains("task failed after committing"));
    assert!(failure.output.contains("review_then_cherry_pick"));
    let guidance = child.guidance();
    assert!(guidance.contains("Parent uncommitted changes are NOT copied"));
    assert!(guidance.contains("Nothing is auto-merged"));
    let review = git(
        repo.path(),
        &["show", "--format=", &commit, "--", "project/data.txt"],
    );
    assert!(review.contains("+reviewed child change"));
    assert_eq!(head(repo.path()), original);
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "primary\n"
    );
    git(repo.path(), &["cherry-pick", &commit]);
    assert_ne!(head(repo.path()), original);
    assert_eq!(
        std::fs::read_to_string(repo.path().join("project/data.txt")).unwrap(),
        "reviewed child change\n"
    );
    assert!(git(repo.path(), &["status", "--porcelain"]).is_empty());
    assert!(child.worktree.is_dir());
    assert_eq!(head(&child.worktree), commit);
}
