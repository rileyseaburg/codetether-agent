use super::WorktreeManager;

#[test]
fn resolves_nested_path_to_workspace_root() {
    let workspace = tempfile::tempdir().expect("workspace");
    git(workspace.path(), &["init"]);
    let nested = workspace.path().join("src/nested");
    std::fs::create_dir_all(&nested).expect("nested");
    let manager = WorktreeManager::for_repo(&nested);
    assert_eq!(manager.repo_path, workspace.path());
    assert_eq!(
        manager.base_dir,
        workspace.path().join(".codetether-worktrees")
    );
}

#[test]
fn legacy_custom_base_is_ignored() {
    let workspace = tempfile::tempdir().expect("workspace");
    let manager = WorktreeManager::with_repo("/tmp/random-worktrees", workspace.path());
    assert_eq!(
        manager.base_dir,
        workspace.path().join(".codetether-worktrees")
    );
}

fn git(cwd: &std::path::Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .status()
        .expect("git");
    assert!(status.success());
}
