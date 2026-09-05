//! Explicit parent HEAD differs from primary HEAD but storage stays primary-rooted.

use super::{fixture, git};

#[tokio::test]
async fn create_from_parent_commit_keeps_primary_storage() {
    let (repo, manager) = fixture();
    let primary_head = git(repo.path(), &["rev-parse", "HEAD"]);
    let parent = manager.create("parent").await.unwrap();
    std::fs::write(parent.path.join("parent.txt"), "committed parent content").unwrap();
    git(&parent.path, &["add", "parent.txt"]);
    git(&parent.path, &["commit", "-m", "parent change"]);
    let parent_head = git(&parent.path, &["rev-parse", "HEAD"]);
    assert_ne!(primary_head, parent_head);
    let child = manager.create_from("child", &parent_head).await.unwrap();
    assert_eq!(child.path, repo.path().join(".codetether-worktrees/child"));
    assert_eq!(child.branch, "codetether/child");
    assert!(child.active);
    assert_eq!(git(&child.path, &["rev-parse", "HEAD"]), parent_head);
    assert_eq!(git(repo.path(), &["rev-parse", "HEAD"]), primary_head);
    assert_eq!(
        std::fs::read_to_string(child.path.join("parent.txt")).unwrap(),
        "committed parent content"
    );
    assert_eq!(manager.worktrees.lock().await.len(), 2);
    let legacy = manager.create("legacy").await.unwrap();
    assert_eq!(git(&legacy.path, &["rev-parse", "HEAD"]), primary_head);
}

#[tokio::test]
async fn create_from_resolves_commit_references() {
    let (repo, manager) = fixture();
    let child = manager.create_from("head-ref", "HEAD").await.unwrap();
    assert_eq!(
        git(&child.path, &["rev-parse", "HEAD"]),
        git(repo.path(), &["rev-parse", "HEAD"])
    );
}
