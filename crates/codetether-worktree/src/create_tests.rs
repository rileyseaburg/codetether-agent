//! Real Git fixtures for explicit-revision worktree allocation.

use super::WorktreeManager;
use std::path::Path;

#[path = "create_failure_tests.rs"]
mod failure;
#[path = "create_invalid_tests.rs"]
mod invalid;
#[path = "create_revision_tests.rs"]
mod revision;

fn git(path: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn fixture() -> (tempfile::TempDir, WorktreeManager) {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init"]);
    git(repo.path(), &["config", "user.name", "Worktree Test"]);
    git(
        repo.path(),
        &["config", "user.email", "worktree@example.invalid"],
    );
    git(repo.path(), &["config", "commit.gpgsign", "false"]);
    git(repo.path(), &["commit", "--allow-empty", "-m", "primary"]);
    let manager = WorktreeManager::for_repo(repo.path()).without_vscode_auto_open();
    (repo, manager)
}
