//! Tests for the repository hook gate on worker commits.

use super::require_installed;
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

#[cfg(unix)]
fn install(root: &Path, name: &str) {
    use std::os::unix::fs::PermissionsExt;
    let hook = root.join(".git/hooks").join(name);
    write(&hook, "#!/bin/sh\nexit 0\n");
    std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}

const CONFIG: &str = "default_install_hook_types: [pre-commit, post-commit]\n";

#[tokio::test]
async fn repository_without_declared_hooks_is_allowed() {
    let dir = repo();
    require_installed(dir.path())
        .await
        .expect("no hooks declared");
}

#[tokio::test]
async fn declared_but_uninstalled_hooks_block_the_commit() {
    let dir = repo();
    write(&dir.path().join(".pre-commit-config.yaml"), CONFIG);
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    let error = require_installed(dir.path())
        .await
        .expect_err("hooks missing")
        .to_string();
    for name in ["pre-commit", "post-commit", "pre-push"] {
        assert!(error.contains(name), "{error}");
    }
}

#[cfg(unix)]
#[tokio::test]
async fn a_non_executable_hook_does_not_count() {
    let dir = repo();
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    write(&dir.path().join(".git/hooks/pre-push"), "#!/bin/sh\n");
    assert!(require_installed(dir.path()).await.is_err());
}

#[cfg(unix)]
#[tokio::test]
async fn fully_installed_hooks_allow_the_commit() {
    let dir = repo();
    write(&dir.path().join(".pre-commit-config.yaml"), CONFIG);
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    for name in ["pre-commit", "post-commit", "pre-push"] {
        install(dir.path(), name);
    }
    require_installed(dir.path())
        .await
        .expect("hooks installed");
}

#[cfg(unix)]
#[tokio::test]
async fn custom_hooks_path_is_honoured() {
    let dir = repo();
    write(&dir.path().join(".githooks/pre-push"), "#!/bin/sh\n");
    let status = Command::new("git")
        .args(["config", "core.hooksPath", ".githooks"])
        .current_dir(dir.path())
        .status()
        .expect("git config");
    assert!(status.success());
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(
        dir.path().join(".githooks/pre-push"),
        std::fs::Permissions::from_mode(0o755),
    )
    .expect("chmod");
    require_installed(dir.path())
        .await
        .expect("hooksPath install");
}
