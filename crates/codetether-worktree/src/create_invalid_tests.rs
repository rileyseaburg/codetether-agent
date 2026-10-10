//! Invalid revisions must not register branches or checkouts.

use super::{fixture, git};

#[tokio::test]
async fn create_from_invalid_revision_does_not_create_branch() {
    let (repo, manager) = fixture();
    let before = git(repo.path(), &["for-each-ref", "--format=%(refname)"]);
    assert!(
        manager
            .create_from("fresh", "missing-revision")
            .await
            .is_err()
    );
    assert_eq!(
        git(repo.path(), &["for-each-ref", "--format=%(refname)"]),
        before
    );
    assert!(!repo.path().join(".codetether-worktrees/fresh").exists());
    assert!(manager.worktrees.lock().await.is_empty());
}
