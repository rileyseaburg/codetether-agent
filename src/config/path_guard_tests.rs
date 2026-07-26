use super::path_guard::is_unsafe_workspace_root;
use std::path::Path;

#[test]
fn rejects_filesystem_root() {
    assert!(is_unsafe_workspace_root(Path::new("/")));
}

#[test]
fn rejects_current_home_directory() {
    let Ok(home) = std::env::var("HOME") else {
        return;
    };
    assert!(is_unsafe_workspace_root(Path::new(&home)));
}

#[test]
fn accepts_normal_project_directory() {
    assert!(!is_unsafe_workspace_root(Path::new("/home/user/project")));
}

#[test]
fn accepts_nested_workspace_directory() {
    assert!(!is_unsafe_workspace_root(Path::new(
        "/home/user/project/crate"
    )));
}
