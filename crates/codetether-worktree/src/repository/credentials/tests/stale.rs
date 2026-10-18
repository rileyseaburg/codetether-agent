//! Credential helper configuration regression.
use super::super::run_git_config_command_with_lock_recovery;
use std::{process::Command, time::Duration};
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
