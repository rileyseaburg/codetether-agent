//! Credential helper configuration regression.
use super::super::configure_repo_git_auth;
use std::process::Command;
#[test]
fn configures_helper_outside_worktree() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo = dir.path();
    let init = Command::new("git")
        .current_dir(repo)
        .args(["init"])
        .output()
        .expect("git init should run");
    assert!(
        init.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let helper = configure_repo_git_auth(repo, "ws-1").expect("configure helper");

    assert!(helper.starts_with(repo.join(".git")));
    assert!(!repo.join(".codetether-git-credential-helper").exists());

    let status = Command::new("git")
        .current_dir(repo)
        .args(["status", "--short"])
        .output()
        .expect("git status should run");
    assert!(
        status.status.success(),
        "git status failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    assert!(!String::from_utf8_lossy(&status.stdout).contains("codetether-git-credential-helper"));
}
