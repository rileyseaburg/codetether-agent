use super::normalize_tool_args;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn normalizes_relative_path_inside_worktree() {
    let root = tempdir().unwrap();
    let mut args = json!({"path": "src/main.rs"});
    normalize_tool_args("read", &mut args, root.path()).unwrap();
    assert!(
        args["path"]
            .as_str()
            .unwrap()
            .starts_with(root.path().to_str().unwrap())
    );
}

#[test]
fn rejects_absolute_path_outside_worktree() {
    let root = tempdir().unwrap();
    let mut args = json!({"path": "/etc/passwd"});
    assert!(normalize_tool_args("read", &mut args, root.path()).is_err());
}

#[test]
fn rejects_parent_traversal_outside_worktree() {
    let root = tempdir().unwrap();
    let mut args = json!({"path": "../escape.txt"});
    assert!(normalize_tool_args("write", &mut args, root.path()).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_symlink_resolution_outside_worktree() {
    let root = tempdir().unwrap();
    std::os::unix::fs::symlink("/etc/passwd", root.path().join("link")).unwrap();
    let mut args = json!({"path": "link"});
    assert!(normalize_tool_args("read", &mut args, root.path()).is_err());
}

#[test]
fn rejects_unrecognized_path_field() {
    let root = tempdir().unwrap();
    let mut args = json!({"target_path": "../escape.txt"});
    assert!(normalize_tool_args("unknown", &mut args, root.path()).is_err());
}
