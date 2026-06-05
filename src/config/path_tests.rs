use super::path::{detect_workspace_root, workspace_data_dir_from};
use tempfile::tempdir;

#[test]
fn detects_workspace_root_using_git_marker() {
    let (repo_root, nested) = temp_repo_nested();
    assert_eq!(
        detect_workspace_root(&nested).as_deref(),
        Some(repo_root.as_path())
    );
}

#[test]
fn workspace_data_dir_defaults_to_workspace_root() {
    let (repo_root, nested) = temp_repo_nested();
    assert_eq!(
        workspace_data_dir_from(&nested),
        repo_root.join(".codetether-agent")
    );
}

#[test]
fn workspace_data_dir_falls_back_to_start_when_not_git_repo() {
    let temp = tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");
    assert_eq!(
        workspace_data_dir_from(&workspace),
        workspace.join(".codetether-agent")
    );
}

fn temp_repo_nested() -> (std::path::PathBuf, std::path::PathBuf) {
    let temp = tempdir().expect("tempdir").into_path();
    let repo_root = temp.join("repo");
    std::fs::create_dir_all(repo_root.join(".git")).expect("create .git");
    let nested = repo_root.join("src").join("nested");
    std::fs::create_dir_all(&nested).expect("create nested");
    (repo_root, nested)
}
