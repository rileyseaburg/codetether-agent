//! Repository credential-helper configuration regression tests.
use super::configure_repo_git_auth;
use crate::a2a::git_credentials::repo_config::run_git_config_command_with_lock_recovery;
use std::process::Command;
use std::time::Duration;

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

#[test]
fn recovers_stale_git_config_lock_before_config_retry() {
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
    let lock_path = repo.join(".git/config.lock");
    std::fs::write(&lock_path, b"stale interrupted writer").expect("write stale lock");

    run_git_config_command_with_lock_recovery(
        repo,
        &["config", "--local", "codetether.workspaceId", "ws-lock"],
        Duration::ZERO,
    )
    .expect("stale config lock should be recovered");

    assert!(!lock_path.exists());
    let value = Command::new("git")
        .current_dir(repo)
        .args(["config", "--local", "--get", "codetether.workspaceId"])
        .output()
        .expect("git config get should run");
    assert!(value.status.success());
    assert_eq!(String::from_utf8_lossy(&value.stdout).trim(), "ws-lock");
}

#[test]
fn refuses_to_remove_fresh_git_config_lock() {
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
    let lock_path = repo.join(".git/config.lock");
    std::fs::write(&lock_path, b"active writer").expect("write fresh lock");

    let error = run_git_config_command_with_lock_recovery(
        repo,
        &["config", "--local", "codetether.workspaceId", "ws-fresh"],
        Duration::from_secs(3600),
    )
    .expect_err("fresh config lock should not be removed");

    assert!(lock_path.exists());
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string().contains("too new to remove safely"))
    );
}
