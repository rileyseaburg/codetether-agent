//! Behavioral coverage for both mux workspace isolation policies.

use crate::mux::isolation::{Isolation, workspace};
use std::path::Path;

#[tokio::test]
async fn git_workspace_is_created_under_managed_storage() {
    let root = tempfile::tempdir().unwrap();
    seed(root.path());
    let isolated = workspace("demo", 2, &root.path().join("service"), Isolation::Worktree)
        .await
        .unwrap();
    assert!(isolated.starts_with(root.path().join(".codetether-worktrees")));
    assert_eq!(isolated.file_name().unwrap(), "service");
    assert_eq!(
        std::fs::read_to_string(isolated.join("file.txt")).unwrap(),
        "base"
    );
}

#[tokio::test]
async fn shared_isolation_reuses_requested_checkout() {
    let root = tempfile::tempdir().unwrap();
    seed(root.path());
    let requested = root.path().join("service");
    let resolved = workspace("demo", 2, &requested, Isolation::Shared)
        .await
        .unwrap();
    assert_eq!(resolved, std::fs::canonicalize(&requested).unwrap());
    assert!(!root.path().join(".codetether-worktrees").exists());
}

fn seed(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "mux@test.invalid"]);
    git(root, &["config", "user.name", "Mux Test"]);
    std::fs::create_dir(root.join("service")).unwrap();
    std::fs::write(root.join("service/file.txt"), "base").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "base"]);
}

fn git(directory: &Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(directory)
        .status()
        .unwrap();
    assert!(status.success());
}
