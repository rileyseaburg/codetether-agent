//! Declared-hook installation against temporary Git repositories.
use super::install_declared_hooks;
use crate::repository::publication::require_installed;
use std::path::Path;
use std::process::Command;

fn repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let status = Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir.path())
        .status()
        .expect("git init");
    assert!(status.success());
    dir
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, text).expect("write");
}

/// Declare `.githooks/pre-push` with an installer script running `body`.
fn declare_pre_push(root: &Path, body: &str) {
    write(&root.join(".githooks/pre-push"), "#!/bin/sh\nexit 0\n");
    write(
        &root.join("scripts/dev/install-pre-push-cache-gate.sh"),
        body,
    );
}

const INSTALLS_PRE_PUSH: &str = "set -eu\nhooks=\"$(git rev-parse --git-path hooks)\"\n\
mkdir -p \"$hooks\"\ninstall -m 755 .githooks/pre-push \"$hooks/pre-push\"\n";

#[tokio::test]
async fn no_declared_hooks_runs_nothing() {
    let dir = repo();
    let installed = install_declared_hooks(dir.path()).await.expect("no hooks");
    assert!(installed.is_empty());
}

#[cfg(unix)]
#[tokio::test]
async fn pre_push_installer_satisfies_the_declared_policy() {
    let dir = repo();
    declare_pre_push(dir.path(), INSTALLS_PRE_PUSH);
    let installed = install_declared_hooks(dir.path()).await.expect("installs");
    assert_eq!(installed, ["pre-push"]);
    require_installed(dir.path())
        .await
        .expect("policy satisfied");
    // Re-preparing an already prepared clone stays successful.
    install_declared_hooks(dir.path())
        .await
        .expect("idempotent");
}

#[tokio::test]
async fn failing_installer_refuses_the_task() {
    let dir = repo();
    declare_pre_push(dir.path(), "echo refusing existing hook >&2\nexit 3\n");
    let error = install_declared_hooks(dir.path()).await.unwrap_err();
    let text = format!("{error:#}");
    assert!(
        text.contains("repository hooks could not be installed"),
        "{text}"
    );
    assert!(text.contains("refusing existing hook"), "{text}");
}

#[tokio::test]
async fn declared_pre_push_without_installer_is_refused() {
    let dir = repo();
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    let error = format!(
        "{:#}",
        install_declared_hooks(dir.path()).await.unwrap_err()
    );
    assert!(
        error.contains("install-pre-push-cache-gate.sh is missing"),
        "{error}"
    );
}

#[tokio::test]
async fn missing_pre_commit_tool_is_refused() {
    let dir = repo();
    write(&dir.path().join(".pre-commit-config.yaml"), "repos: []\n");
    let error = super::installer::run(
        dir.path(),
        "codetether-test-missing-pre-commit",
        &["install"],
        std::time::Duration::from_secs(5),
    )
    .await
    .unwrap_err();
    assert!(error.to_string().contains("could not start"), "{error}");
}

#[cfg(unix)]
#[tokio::test]
async fn slow_installer_times_out() {
    let dir = repo();
    let error = super::installer::run(
        dir.path(),
        "sh",
        &["-c", "sleep 5"],
        std::time::Duration::from_millis(200),
    )
    .await
    .unwrap_err();
    assert!(error.to_string().contains("timed out"), "{error}");
}
