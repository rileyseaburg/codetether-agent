use super::directory;

#[test]
fn completes_unique_directory_with_spaces() {
    let workspace = tempfile::tempdir().unwrap();
    std::fs::create_dir(workspace.path().join("alpha one")).unwrap();
    std::fs::write(workspace.path().join("alpha-file"), "ignored").unwrap();
    let result = directory("cd al", workspace.path()).unwrap();
    assert_eq!(result.matches.len(), 1);
    assert_eq!(
        result.line,
        format!("cd {}/alpha one/", workspace.path().display())
    );
}

#[test]
fn completes_shared_directory_prefix() {
    let workspace = tempfile::tempdir().unwrap();
    std::fs::create_dir(workspace.path().join("folder-a")).unwrap();
    std::fs::create_dir(workspace.path().join("folder-b")).unwrap();
    let result = directory("new fol", workspace.path()).unwrap();
    assert_eq!(result.matches.len(), 2);
    assert_eq!(
        result.line,
        format!("new {}/folder-", workspace.path().display())
    );
}

#[test]
fn leaves_non_path_commands_unchanged() {
    let result = directory("select 1", std::path::Path::new("/")).unwrap();
    assert_eq!(result.line, "select 1");
    assert!(result.matches.is_empty());
}

#[test]
fn empty_path_lists_active_workspace_directories() {
    let workspace = tempfile::tempdir().unwrap();
    std::fs::create_dir(workspace.path().join("child")).unwrap();
    let result = directory("cd ", workspace.path()).unwrap();
    assert_eq!(
        result.matches,
        vec![workspace.path().join("child").display().to_string()]
    );
}
