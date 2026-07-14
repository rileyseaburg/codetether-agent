use super::WorktreeManager;
use std::fs;

mod fixture;
use fixture::Fixture;

#[tokio::test]
async fn create_inherits_ignored_package_dependencies() {
    let fixture = Fixture::new();
    let manager = WorktreeManager::for_repo(fixture.root()).without_vscode_auto_open();
    let worktree = manager.create("automatic-dependencies").await.unwrap();
    assert!(
        fs::symlink_metadata(worktree.path.join("ui/node_modules"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn manager_links_ignored_package_dependencies() {
    let fixture = Fixture::new();
    let manager = WorktreeManager::for_repo(fixture.root());
    assert_eq!(
        manager
            .prepare_node_dependencies(&fixture.worktree())
            .unwrap(),
        1
    );
    assert!(
        fs::symlink_metadata(fixture.worktree().join("ui/node_modules"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn manager_refuses_unignored_destination() {
    let fixture = Fixture::new();
    fs::write(fixture.worktree().join(".gitignore"), "").unwrap();
    let manager = WorktreeManager::for_repo(fixture.root());
    let error = manager
        .prepare_node_dependencies(&fixture.worktree())
        .unwrap_err();
    assert!(error.to_string().contains("unignored"));
}
