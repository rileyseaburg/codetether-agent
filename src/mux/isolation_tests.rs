use super::primary_checkout;
use std::path::Path;

#[test]
fn managed_checkout_resolves_to_primary_repository() {
    let repo = Path::new("/repo/.codetether-worktrees/mux-work-1-abcd");
    assert_eq!(primary_checkout(repo), Path::new("/repo"));
}

#[test]
fn primary_repository_is_unchanged() {
    assert_eq!(primary_checkout(Path::new("/repo")), Path::new("/repo"));
}

#[tokio::test]
async fn git_workspace_is_created_under_managed_storage() {
    let root = tempfile::tempdir().unwrap();
    git(root.path(), &["init", "-q"]);
    git(root.path(), &["config", "user.email", "mux@test.invalid"]);
    git(root.path(), &["config", "user.name", "Mux Test"]);
    std::fs::create_dir(root.path().join("service")).unwrap();
    std::fs::write(root.path().join("service/file.txt"), "base").unwrap();
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-qm", "base"]);

    let isolated = super::workspace("demo", 2, &root.path().join("service"))
        .await
        .unwrap();
    assert!(isolated.starts_with(root.path().join(".codetether-worktrees")));
    assert_eq!(isolated.file_name().unwrap(), "service");
    assert_eq!(
        std::fs::read_to_string(isolated.join("file.txt")).unwrap(),
        "base"
    );
}

fn git(directory: &Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(directory)
        .status()
        .unwrap();
    assert!(status.success());
}
