//! `prepare_git` refuses tasks whose declared hooks cannot be installed.
use super::prepare_git;
use crate::task_config::build_task_context;
use std::path::Path;
use std::process::Command;

fn repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let status = Command::new("git")
        .args(["init", "-q", "-b", "main"])
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

fn hooks(root: &Path) -> std::path::PathBuf {
    root.join(".git/hooks")
}

const INSTALLS_PRE_PUSH: &str = "set -eu\nhooks=\"$(git rev-parse --git-path hooks)\"\n\
mkdir -p \"$hooks\"\ninstall -m 755 .githooks/pre-push \"$hooks/pre-push\"\n";

#[cfg(unix)]
#[tokio::test]
async fn installs_declared_hooks_then_provenance_hook() {
    let dir = repo();
    write(
        &dir.path().join(".githooks/pre-push"),
        "#!/bin/sh\nexit 0\n",
    );
    write(
        &dir.path()
            .join("scripts/dev/install-pre-push-cache-gate.sh"),
        INSTALLS_PRE_PUSH,
    );
    let context = build_task_context(&serde_json::json!({}), "task");
    prepare_git("task", Some(dir.path()), None, &context)
        .await
        .expect("prepared");
    assert!(hooks(dir.path()).join("pre-push").is_file());
    let commit_msg = std::fs::read_to_string(hooks(dir.path()).join("commit-msg")).unwrap();
    assert_eq!(commit_msg, codetether_provenance::commit_msg_hook_script());
    // Re-preparing the same clone succeeds.
    prepare_git("task", Some(dir.path()), None, &context)
        .await
        .expect("idempotent");
}

#[tokio::test]
async fn failing_hook_installer_refuses_the_task() {
    let dir = repo();
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    write(
        &dir.path()
            .join("scripts/dev/install-pre-push-cache-gate.sh"),
        "exit 1\n",
    );
    let context = build_task_context(&serde_json::json!({}), "task");
    let error = prepare_git("task", Some(dir.path()), None, &context)
        .await
        .unwrap_err();
    let text = format!("{error:#}");
    assert!(
        text.contains("repository hooks could not be installed"),
        "{text}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn repository_owned_commit_msg_hook_is_kept() {
    let dir = repo();
    let owned = "#!/bin/sh\n# repository commit-msg\nexit 0\n";
    write(&hooks(dir.path()).join("commit-msg"), owned);
    let context = build_task_context(&serde_json::json!({}), "task");
    prepare_git("task", Some(dir.path()), None, &context)
        .await
        .expect("prepared");
    let commit_msg = std::fs::read_to_string(hooks(dir.path()).join("commit-msg")).unwrap();
    assert_eq!(commit_msg, owned);
}
