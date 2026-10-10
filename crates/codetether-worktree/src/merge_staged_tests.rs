//! Tests for no-op merges (a worktree branch with no new commits).

use super::WorktreeManager;
use std::process::Command;

fn git(dir: &std::path::Path, args: &[&str]) {
    let ok = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git runs")
        .status
        .success();
    assert!(ok, "git {args:?} failed");
}

fn init_repo(dir: &std::path::Path) {
    git(dir, &["init", "-q"]);
    git(dir, &["config", "user.email", "t@e.st"]);
    git(dir, &["config", "user.name", "Test"]);
    std::fs::write(dir.join("README.md"), "seed\n").unwrap();
    git(dir, &["add", "."]);
    git(dir, &["commit", "-q", "-m", "seed"]);
}

#[tokio::test]
async fn merge_with_no_changes_succeeds_as_noop() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();
    init_repo(repo);

    let mgr = WorktreeManager::for_repo(repo).without_vscode_auto_open();
    let wt = mgr.create("noop").await.expect("worktree created");
    // The worktree branch has no new commits relative to base.
    let result = mgr.merge(&wt.name).await.expect("merge should not error");

    assert!(result.success, "no-op merge should report success");
    assert_eq!(result.files_changed, 0);
    assert!(result.summary.contains("no changes"));
}
