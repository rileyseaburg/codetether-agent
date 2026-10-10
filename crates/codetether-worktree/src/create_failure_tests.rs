//! Fail-closed revision creation and legacy branch-reuse compatibility.

use super::{fixture, git};

#[tokio::test]
async fn create_from_invalid_revision_never_reuses_existing_branch() {
    let (repo, manager) = fixture();
    git(repo.path(), &["branch", "codetether/existing"]);
    let before = git(repo.path(), &["worktree", "list", "--porcelain"]);
    for revision in ["missing-revision", "", "--detach", "HEAD^{tree}"] {
        let error = manager.create_from("existing", revision).await.unwrap_err();
        assert!(error.to_string().contains("Invalid worktree start point"));
        assert!(!repo.path().join(".codetether-worktrees/existing").exists());
        assert!(manager.worktrees.lock().await.is_empty());
        assert_eq!(
            git(repo.path(), &["worktree", "list", "--porcelain"]),
            before
        );
    }
    let legacy = manager.create("existing").await.unwrap();
    assert_eq!(
        git(&legacy.path, &["rev-parse", "HEAD"]),
        git(repo.path(), &["rev-parse", "HEAD"])
    );
}

#[tokio::test]
async fn create_from_existing_branch_fails_without_reuse_or_reset() {
    let (repo, manager) = fixture();
    git(repo.path(), &["branch", "codetether/existing"]);
    let original = git(repo.path(), &["rev-parse", "codetether/existing"]);
    git(
        repo.path(),
        &["commit", "--allow-empty", "-m", "newer primary"],
    );
    assert!(manager.create_from("existing", "HEAD").await.is_err());
    assert_eq!(
        git(repo.path(), &["rev-parse", "codetether/existing"]),
        original
    );
    assert!(!repo.path().join(".codetether-worktrees/existing").exists());
    assert!(manager.worktrees.lock().await.is_empty());
}
