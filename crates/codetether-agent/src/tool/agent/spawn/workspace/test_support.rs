//! Temporary Git repositories and checked commands for allocation tests.

use std::path::Path;

pub(super) fn git(path: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .expect("execute fixture git command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

pub(super) fn fixture() -> tempfile::TempDir {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init"]);
    git(
        repo.path(),
        &["config", "user.name", "Child Workspace Test"],
    );
    git(
        repo.path(),
        &["config", "user.email", "child@example.invalid"],
    );
    git(repo.path(), &["config", "commit.gpgsign", "false"]);
    std::fs::create_dir(repo.path().join("project")).unwrap();
    std::fs::write(repo.path().join("project/data.txt"), "primary\n").unwrap();
    std::fs::write(repo.path().join(".gitignore"), ".codetether-worktrees/\n").unwrap();
    git(repo.path(), &["add", "project/data.txt", ".gitignore"]);
    git(repo.path(), &["commit", "-m", "primary fixture"]);
    repo
}

pub(super) fn head(path: &Path) -> String {
    git(path, &["rev-parse", "HEAD"])
}

pub(super) fn commit_data(path: &Path, content: &str) -> String {
    std::fs::write(path.join("project/data.txt"), content).unwrap();
    git(path, &["add", "project/data.txt"]);
    git(path, &["commit", "-m", "child task change"]);
    head(path)
}
