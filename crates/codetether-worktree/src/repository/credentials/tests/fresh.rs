//! Credential helper configuration regression.
use super::super::run_git_config_command_with_lock_recovery;
use std::{process::Command, time::Duration};
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
