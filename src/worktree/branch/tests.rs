use super::WorktreeManager;
use std::{path::Path, process::Command};

#[tokio::test]
async fn branch_deletion_preserves_unmerged_commits() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path();
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("seed"), "seed").unwrap();
    git(repo, &["add", "seed"]);
    git(repo, &["commit", "-q", "-m", "seed"]);
    git(repo, &["checkout", "-q", "-b", "codetether/unmerged"]);
    std::fs::write(repo.join("change"), "change").unwrap();
    git(repo, &["add", "change"]);
    git(repo, &["commit", "-q", "-m", "change"]);
    git(repo, &["checkout", "-q", "main"]);

    let deleted = WorktreeManager::delete_branch(repo, "codetether/unmerged").await;

    assert!(!deleted);
    git(repo, &["rev-parse", "--verify", "codetether/unmerged"]);
}

fn git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
