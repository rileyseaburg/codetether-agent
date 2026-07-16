use super::{WorktreeManager, cleanup_remove::RemoveOutcome};
use std::{path::Path, process::Command};

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

#[tokio::test]
async fn failed_git_removal_preserves_checkout() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path();
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("seed"), "seed").unwrap();
    git(repo, &["add", "seed"]);
    git(repo, &["commit", "-q", "-m", "seed"]);
    let manager = WorktreeManager::for_repo(repo).without_vscode_auto_open();
    let worktree = manager.create("locked").await.unwrap();
    git(repo, &["worktree", "lock", worktree.path.to_str().unwrap()]);

    let outcome = manager.remove_worktree(&worktree).await;

    assert_eq!(outcome, RemoveOutcome::Failed);
    assert!(worktree.path.exists());
}
