//! Invalid source directories never silently fall back to shared writes.

use super::super::allocate;
use super::support::{fixture, git};

#[tokio::test]
async fn non_git_directory_fails_closed_without_creating_storage() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("sentinel"), "untouched").unwrap();
    let error = allocate(directory.path())
        .await
        .err()
        .expect("non-Git error");
    assert!(error.to_string().contains("requires a Git checkout"));
    assert!(!directory.path().join(".codetether-worktrees").exists());
    assert_eq!(
        std::fs::read_to_string(directory.path().join("sentinel")).unwrap(),
        "untouched"
    );
}

#[tokio::test]
async fn uncommitted_working_directory_fails_and_retains_managed_checkout() {
    let repo = fixture();
    let requested = repo.path().join("untracked-directory");
    std::fs::create_dir(&requested).unwrap();
    let error = allocate(&requested)
        .await
        .err()
        .expect("missing committed cwd");
    assert!(error.to_string().contains("absent from committed HEAD"));
    assert!(error.to_string().contains("checkout retained at"));
    let storage = repo.path().join(".codetether-worktrees");
    let retained: Vec<_> = std::fs::read_dir(&storage).unwrap().collect();
    assert_eq!(retained.len(), 1);
    let child = retained[0].as_ref().unwrap().path();
    assert!(child.join("project/data.txt").exists());
    assert!(
        git(repo.path(), &["worktree", "list", "--porcelain"])
            .contains(&child.display().to_string())
    );
    assert!(requested.is_dir());
}
