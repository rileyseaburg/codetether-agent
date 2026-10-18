//! Subprocess arguments are shell-free and timeout cleanup covers descendants.

use super::*;
use std::time::Duration;

#[tokio::test]
async fn command_preserves_paths_with_spaces_and_reports_exit_errors() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("output with spaces.png");
    let program = script(
        dir.path(),
        "program with spaces",
        "test \"$1\" = --silent\nprintf ok > \"$2\"",
    );
    run(&program, &target, Duration::from_secs(2))
        .await
        .unwrap();
    assert_eq!(std::fs::read(&target).unwrap(), b"ok");
    let program = script(dir.path(), "denied", "exit 7");
    let error = run(&program, &target, Duration::from_secs(2))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("7"));
}

async fn run(program: &Path, target: &Path, limit: Duration) -> anyhow::Result<()> {
    command::run(Session::X11, Backend::Scrot, program, target, limit).await
}

#[tokio::test]
async fn command_timeout_kills_descendants_before_they_can_write() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("must not exist.png");
    let program = script(
        dir.path(),
        "slow",
        "(sleep 0.3; printf leaked > \"$2\") &\nwait",
    );
    let error = run(&program, &target, Duration::from_millis(50))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("timed out"));
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert!(
        !target.exists(),
        "a descendant survived capture cancellation"
    );
}
