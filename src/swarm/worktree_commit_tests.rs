use super::prepare;
use crate::worktree::WorktreeInfo;
use std::{path::Path, process::Command};

fn git(path: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[tokio::test]
async fn commits_dirty_worktree_changes() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    std::fs::write(dir.path().join("file.txt"), "before").unwrap();
    git(dir.path(), &["add", "."]);
    git(
        dir.path(),
        &[
            "-c",
            "user.name=test",
            "-c",
            "user.email=test@example.com",
            "commit",
            "-qm",
            "seed",
        ],
    );
    std::fs::write(dir.path().join("file.txt"), "after").unwrap();
    let info = WorktreeInfo {
        name: "test".into(),
        path: dir.path().into(),
        branch: "main".into(),
        active: true,
    };
    prepare(&info, "task-1").await.unwrap();
    assert!(git(dir.path(), &["status", "--porcelain"]).is_empty());
    assert!(git(dir.path(), &["log", "-1", "--pretty=%s"]).contains("task-1"));
}
